//! Streaming download of a file, verified against its expected size and SHA-256.

use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use crate::error::{Error, Result};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
/// A stalled connection is abandoned after this long without receiving data.
const READ_TIMEOUT: Duration = Duration::from_secs(30);
const BUFFER_SIZE: usize = 256 * 1024;
const PROGRESS_INTERVAL: Duration = Duration::from_millis(150);

/// What a download must produce to be accepted.
pub(super) struct ExpectedFile<'a> {
    pub url: &'a str,
    pub size: u64,
    pub sha256: &'a str,
}

/// Streams `expected.url` into `destination`, hashing while writing.
///
/// `on_progress` receives the number of bytes downloaded so far, throttled to
/// a few calls per second. The download fails with [`Error::Cancelled`] as soon
/// as `cancel` is set, and with a verification error if the size or checksum
/// differ from what the catalog promises.
pub(super) fn download_verified(
    expected: &ExpectedFile<'_>,
    destination: &File,
    cancel: &AtomicBool,
    mut on_progress: impl FnMut(u64),
) -> Result<()> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_connect(Some(CONNECT_TIMEOUT))
        .timeout_per_read(Some(READ_TIMEOUT))
        .user_agent(concat!("pirates-parrot/", env!("CARGO_PKG_VERSION")))
        .build()
        .into();

    let mut response = agent.get(expected.url).call()?;
    let mut body = response.body_mut().as_reader();
    let mut writer = BufWriter::new(destination);
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut received: u64 = 0;
    let mut last_report = Instant::now();

    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(Error::Cancelled);
        }
        let read = body.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        let chunk = &buffer[..read];
        hasher.update(chunk);
        writer.write_all(chunk)?;
        received += read as u64;
        // A server sending more than expected cannot be the right file; stop
        // before it fills the disk.
        if received > expected.size {
            return Err(Error::WrongDownloadSize {
                expected: expected.size,
                received,
            });
        }

        if last_report.elapsed() >= PROGRESS_INTERVAL {
            on_progress(received);
            last_report = Instant::now();
        }
    }
    writer.flush()?;
    on_progress(received);

    if received != expected.size {
        return Err(Error::WrongDownloadSize {
            expected: expected.size,
            received,
        });
    }
    if !to_hex(&hasher.finalize()).eq_ignore_ascii_case(expected.sha256) {
        return Err(Error::ChecksumMismatch);
    }
    destination.sync_all()?;
    Ok(())
}

fn to_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;

    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut hex, byte| {
            // Writing to a String cannot fail.
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_encodes_sha256_digest() {
        let digest = Sha256::digest(b"abc");
        assert_eq!(
            to_hex(&digest),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
