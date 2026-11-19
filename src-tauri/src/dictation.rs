//! Starts and stops dictations in response to the shortcut and the widget,
//! and hands the final transcript to [`crate::output`].

use std::sync::mpsc::{self, Sender};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::thread;

use parrot_core::{Dictation, DictationEvent, DictationOptions};
use serde::Serialize;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Manager};

use crate::engine::Engine;
use crate::error::AppError;
use crate::events::{self, DictationResult};
use crate::output;
use crate::settings::{Settings, SettingsFile};
use crate::windows::widget;

/// What the dictation machinery is doing right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DictationPhase {
    #[default]
    Idle,
    /// The microphone is recording.
    Listening,
    /// Recording stopped; the model is transcribing the last words.
    Finishing,
}

/// Something the user asked for, from the shortcut, the widget or the settings.
#[derive(Debug, Clone, Copy)]
pub enum Request {
    Start,
    Finish,
    /// Starts when idle, finishes while listening.
    Toggle,
    /// Stops listening and throws the transcript away.
    Cancel,
}

#[derive(Debug)]
pub struct DictationController {
    inner: Mutex<Inner>,
    requests: Sender<Request>,
}

#[derive(Debug, Default)]
struct Inner {
    phase: DictationPhase,
    session: Option<Dictation>,
}

impl DictationController {
    /// Creates the controller along with the thread that serves its requests.
    /// It must be managed as app state before the first request is made.
    ///
    /// Requests are handled one at a time, in the order they were made, and
    /// never on the caller's thread: opening a microphone can take seconds (a
    /// Bluetooth headset, for one), and the shortcut handler and the UI must
    /// not wait for it.
    pub fn new(app: &AppHandle) -> Result<Self, AppError> {
        let (requests, received) = mpsc::channel();
        let app = app.clone();
        thread::Builder::new()
            .name("dictation-requests".into())
            .spawn(move || {
                for request in received {
                    app.state::<DictationController>().handle(&app, request);
                }
            })?;
        Ok(Self {
            inner: Mutex::default(),
            requests,
        })
    }

    pub fn phase(&self) -> DictationPhase {
        self.lock().phase
    }

    /// Queues a request; it is carried out in the background.
    pub fn request(&self, request: Request) {
        if self.requests.send(request).is_err() {
            log::error!("dictation requests are no longer served; dropped {request:?}");
        }
    }

    fn handle(&self, app: &AppHandle, request: Request) {
        match request {
            Request::Start => self.start(app),
            Request::Finish => self.finish(app),
            Request::Toggle => match self.phase() {
                DictationPhase::Idle => self.start(app),
                DictationPhase::Listening => self.finish(app),
                DictationPhase::Finishing => {}
            },
            Request::Cancel => self.cancel(app),
        }
    }

    /// Starts listening if nothing else is going on. Problems are reported to
    /// the UI as a failed [`DictationResult`].
    fn start(&self, app: &AppHandle) {
        let mut inner = self.lock();
        if inner.phase != DictationPhase::Idle {
            return;
        }
        // The parrot always shows what it hears, even when the user hid it.
        if let Err(error) = widget::reveal(app) {
            log::warn!("could not reveal the parrot: {error}");
        }
        match open_session(app) {
            Ok(session) => {
                inner.session = Some(session);
                set_phase(app, &mut inner, DictationPhase::Listening);
            }
            Err(error) => {
                log::warn!("could not start dictation: {error}");
                let message = error.to_string();
                events::dictation_result(app, &DictationResult::Failed { message });
            }
        }
    }

    /// Stops listening and delivers the transcript once the model has caught
    /// up. Returns immediately; the work happens on a background thread.
    fn finish(&self, app: &AppHandle) {
        let mut inner = self.lock();
        let Some(session) = inner.session.take() else {
            return;
        };
        set_phase(app, &mut inner, DictationPhase::Finishing);
        drop(inner);

        let app = app.clone();
        spawn_blocking(move || {
            let result = match session.finish() {
                Ok(text) => deliver(&app, text),
                Err(error) => DictationResult::Failed {
                    message: error.to_string(),
                },
            };
            events::dictation_result(&app, &result);
            let controller = app.state::<DictationController>();
            set_phase(&app, &mut controller.lock(), DictationPhase::Idle);
        });
    }

    fn cancel(&self, app: &AppHandle) {
        let mut inner = self.lock();
        let Some(session) = inner.session.take() else {
            return;
        };
        set_phase(app, &mut inner, DictationPhase::Idle);
        drop(inner);
        session.cancel();
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        // `Inner` is updated with plain assignments, so it stays consistent
        // even if a holder of the lock panicked.
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

fn open_session(app: &AppHandle) -> Result<Dictation, AppError> {
    let transcriber = app
        .state::<Engine>()
        .transcriber()
        .ok_or(AppError::ModelNotReady)?;
    let settings: Settings = app.state::<SettingsFile>().get();
    let options = DictationOptions {
        language: settings.language,
        input_device: settings.microphone,
    };

    let app = app.clone();
    let session = Dictation::start(&transcriber, &options, move |event| match event {
        DictationEvent::Transcript(text) => events::transcript(&app, &text),
        DictationEvent::Level(level) => events::level(&app, level),
        DictationEvent::DeviceLost(reason) => {
            log::warn!("microphone lost during dictation: {reason}");
            app.state::<DictationController>().request(Request::Finish);
        }
    })?;
    Ok(session)
}

fn deliver(app: &AppHandle, text: String) -> DictationResult {
    if text.is_empty() {
        return DictationResult::Empty;
    }
    let mode = app.state::<SettingsFile>().get().output;
    match output::deliver(app, &text, mode) {
        Ok(pasted) => DictationResult::Delivered { text, pasted },
        Err(error) => {
            log::error!("could not deliver transcript: {error}");
            DictationResult::Failed {
                message: error.to_string(),
            }
        }
    }
}

fn set_phase(app: &AppHandle, inner: &mut Inner, phase: DictationPhase) {
    if inner.phase != phase {
        inner.phase = phase;
        events::dictation_phase(app, phase);
    }
}
