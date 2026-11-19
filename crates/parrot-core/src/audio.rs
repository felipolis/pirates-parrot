//! Microphone capture.
//!
//! A cpal stream is not `Send` on every platform, so each capture runs on its
//! own thread that owns the stream for its whole life and forwards mono audio
//! through a channel.

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    Device, DeviceId, ErrorKind, FromSample, Sample, SampleFormat, SizedSample, StreamConfig,
};
use serde::Serialize;

use crate::error::{Error, Result};

/// A microphone the user can pick.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputDevice {
    /// Stable identifier, suitable for saving in settings.
    pub id: String,
    pub name: String,
    pub is_default: bool,
}

/// Lists the microphones currently connected.
pub fn list_input_devices() -> Result<Vec<InputDevice>> {
    let host = cpal::default_host();
    let default_id = host
        .default_input_device()
        .and_then(|device| device.id().ok());

    let mut devices = Vec::new();
    for device in host.input_devices()? {
        let (Ok(id), Ok(description)) = (device.id(), device.description()) else {
            continue;
        };
        devices.push(InputDevice {
            is_default: default_id.as_ref() == Some(&id),
            id: id.to_string(),
            name: description.name().to_owned(),
        });
    }
    Ok(devices)
}

#[derive(Debug)]
pub(crate) enum CaptureMessage {
    /// Mono samples in `-1.0..=1.0` at [`AudioCapture::sample_rate`].
    Audio(Vec<f32>),
    /// The device stopped working, for example because it was unplugged.
    Failed(String),
}

/// A running microphone capture. Dropping it stops the microphone, after
/// which the receiving end of the channel disconnects.
#[derive(Debug)]
pub(crate) struct AudioCapture {
    sample_rate: u32,
    stop: Option<Sender<()>>,
    thread: Option<JoinHandle<()>>,
}

impl AudioCapture {
    /// Starts capturing from the microphone identified by `device_id`, or the
    /// system default, sending audio to `sink`. Returns once the microphone is
    /// recording, or with the error that prevented it.
    pub(crate) fn start(device_id: Option<&str>, sink: Sender<CaptureMessage>) -> Result<Self> {
        let device_id = device_id.map(str::to_owned);
        let (ready_tx, ready_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel::<()>();

        let thread = thread::Builder::new()
            .name("audio-capture".to_owned())
            .spawn(move || {
                let (stream, sample_rate) = match find_device(device_id.as_deref())
                    .and_then(|device| open_stream(&device, sink))
                {
                    Ok(opened) => opened,
                    Err(error) => {
                        let _ = ready_tx.send(Err(error));
                        return;
                    }
                };
                if let Err(error) = stream.play() {
                    let _ = ready_tx.send(Err(error.into()));
                    return;
                }
                let _ = ready_tx.send(Ok(sample_rate));
                // Blocks until a stop is requested or the handle is dropped.
                let _ = stop_rx.recv();
            })?;

        match ready_rx.recv() {
            Ok(Ok(sample_rate)) => Ok(Self {
                sample_rate,
                stop: Some(stop_tx),
                thread: Some(thread),
            }),
            Ok(Err(error)) => {
                join_capture_thread(thread);
                Err(error)
            }
            Err(_) => {
                join_capture_thread(thread);
                Err(Error::WorkerPanicked)
            }
        }
    }

    /// Rate of the samples sent to the sink.
    pub(crate) fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

impl Drop for AudioCapture {
    fn drop(&mut self) {
        drop(self.stop.take());
        if let Some(thread) = self.thread.take() {
            join_capture_thread(thread);
        }
    }
}

/// Waits for the capture thread to end, logging it if it crashed.
fn join_capture_thread(thread: JoinHandle<()>) {
    if thread.join().is_err() {
        log::error!("the audio capture thread panicked");
    }
}

fn find_device(device_id: Option<&str>) -> Result<Device> {
    let host = cpal::default_host();
    match device_id {
        Some(id) => {
            let id: DeviceId = id.parse()?;
            host.device_by_id(&id).ok_or(Error::InputDeviceNotFound)
        }
        None => host.default_input_device().ok_or(Error::NoInputDevice),
    }
}

/// Opens the device in its preferred format and returns the paused stream with
/// the sample rate it delivers.
fn open_stream(device: &Device, sink: Sender<CaptureMessage>) -> Result<(cpal::Stream, u32)> {
    let supported = device.default_input_config()?;
    let format = supported.sample_format();
    let config = StreamConfig::from(supported);

    let stream = match format {
        SampleFormat::F32 => build_stream::<f32>(device, config, sink),
        SampleFormat::F64 => build_stream::<f64>(device, config, sink),
        SampleFormat::I8 => build_stream::<i8>(device, config, sink),
        SampleFormat::I16 => build_stream::<i16>(device, config, sink),
        SampleFormat::I24 => build_stream::<cpal::I24>(device, config, sink),
        SampleFormat::I32 => build_stream::<i32>(device, config, sink),
        SampleFormat::U8 => build_stream::<u8>(device, config, sink),
        SampleFormat::U16 => build_stream::<u16>(device, config, sink),
        SampleFormat::U32 => build_stream::<u32>(device, config, sink),
        other => Err(Error::UnsupportedSampleFormat(other.to_string())),
    }?;
    Ok((stream, config.sample_rate))
}

fn build_stream<T>(
    device: &Device,
    config: StreamConfig,
    sink: Sender<CaptureMessage>,
) -> Result<cpal::Stream>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = usize::from(config.channels);
    let error_sink = sink.clone();

    let on_data = move |data: &[T], _: &cpal::InputCallbackInfo| {
        // The receiver only goes away while the capture is being torn down.
        let _ = sink.send(CaptureMessage::Audio(downmix(data, channels)));
    };
    let on_error = move |error: cpal::Error| {
        if is_recoverable(error.kind()) {
            log::warn!("microphone glitch: {error}");
        } else {
            log::error!("microphone stopped: {error}");
            let _ = error_sink.send(CaptureMessage::Failed(error.to_string()));
        }
    };

    Ok(device.build_input_stream(config, on_data, on_error, None)?)
}

/// Errors after which the stream keeps delivering audio.
fn is_recoverable(kind: ErrorKind) -> bool {
    matches!(
        kind,
        ErrorKind::DeviceChanged | ErrorKind::Xrun | ErrorKind::RealtimeDenied
    )
}

/// Averages interleaved frames into mono `f32` samples.
fn downmix<T>(interleaved: &[T], channels: usize) -> Vec<f32>
where
    T: Sample,
    f32: FromSample<T>,
{
    interleaved
        .chunks_exact(channels.max(1))
        .map(|frame| {
            frame
                .iter()
                .map(|&sample| f32::from_sample(sample))
                .sum::<f32>()
                / frame.len() as f32
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn downmix_averages_channels() {
        assert_eq!(downmix(&[0.5_f32, -0.5, 1.0, 0.0], 2), [0.0, 0.5]);
    }

    #[test]
    fn downmix_converts_integer_samples() {
        assert_eq!(downmix(&[i16::MIN, 0], 1), [-1.0, 0.0]);
    }

    #[test]
    fn downmix_keeps_mono_untouched() {
        assert_eq!(downmix(&[0.25_f32, -0.75], 1), [0.25, -0.75]);
    }
}
