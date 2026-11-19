//! The folder where speech models are installed: downloading, unpacking,
//! finding and removing them.

use std::fs::{self, File};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use bzip2::read::BzDecoder;
use serde::Serialize;

use super::catalog::{ModelId, ModelSpec};
use super::download::{ExpectedFile, download_verified};
use crate::error::{Error, Result};

const ENCODER: &str = "encoder.int8.onnx";
const DECODER: &str = "decoder.int8.onnx";
const JOINER: &str = "joiner.int8.onnx";
const TOKENS: &str = "tokens.txt";
const REQUIRED_FILES: [&str; 4] = [ENCODER, DECODER, JOINER, TOKENS];

/// Prefix of the temporary files and folders an installation creates, so
/// leftovers from an interrupted install can be recognised and removed.
const TEMP_PREFIX: &str = ".partial-";

/// Paths of the files that make up an installed model.
#[derive(Debug, Clone)]
pub struct ModelFiles {
    pub encoder: PathBuf,
    pub decoder: PathBuf,
    pub joiner: PathBuf,
    pub tokens: PathBuf,
}

impl ModelFiles {
    /// Expected locations of the model files inside `dir`.
    pub fn in_dir(dir: &Path) -> Self {
        Self {
            encoder: dir.join(ENCODER),
            decoder: dir.join(DECODER),
            joiner: dir.join(JOINER),
            tokens: dir.join(TOKENS),
        }
    }

    fn all_exist(&self) -> bool {
        [&self.encoder, &self.decoder, &self.joiner, &self.tokens]
            .iter()
            .all(|path| path.is_file())
    }
}

/// Stage reached by a model installation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "stage", rename_all = "camelCase")]
pub enum InstallProgress {
    Downloading { downloaded: u64, total: u64 },
    Extracting,
}

/// Directory holding the installed speech models, one folder per model.
#[derive(Debug, Clone)]
pub struct ModelStore {
    root: PathBuf,
}

impl ModelStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Files of `id` if the model is fully installed.
    pub fn installed_files(&self, id: ModelId) -> Option<ModelFiles> {
        let files = ModelFiles::in_dir(&self.model_dir(id.spec()));
        files.all_exist().then_some(files)
    }

    pub fn is_installed(&self, id: ModelId) -> bool {
        self.installed_files(id).is_some()
    }

    /// Downloads, verifies and unpacks `id`.
    ///
    /// The model only becomes visible once every file is in place, so an
    /// interrupted or cancelled install never leaves a half-installed model.
    pub fn install(
        &self,
        id: ModelId,
        cancel: &AtomicBool,
        mut on_progress: impl FnMut(InstallProgress),
    ) -> Result<ModelFiles> {
        let spec = id.spec();
        fs::create_dir_all(&self.root)?;
        self.remove_leftovers()?;

        let mut archive = tempfile::Builder::new()
            .prefix(TEMP_PREFIX)
            .suffix(".tar.bz2")
            .tempfile_in(&self.root)?;
        let expected = ExpectedFile {
            url: spec.archive_url,
            size: spec.archive_size,
            sha256: spec.archive_sha256,
        };
        download_verified(&expected, archive.as_file(), cancel, |downloaded| {
            on_progress(InstallProgress::Downloading {
                downloaded,
                total: spec.archive_size,
            });
        })?;

        on_progress(InstallProgress::Extracting);
        let archive_file = archive.as_file_mut();
        archive_file.seek(SeekFrom::Start(0))?;
        let staging = tempfile::Builder::new()
            .prefix(TEMP_PREFIX)
            .tempdir_in(&self.root)?;
        extract_model(&*archive_file, staging.path(), cancel)?;

        let target = self.model_dir(spec);
        if target.exists() {
            fs::remove_dir_all(&target)?;
        }
        fs::rename(staging.keep(), &target)?;
        Ok(ModelFiles::in_dir(&target))
    }

    /// Deletes an installed model. Removing a model that is not installed is
    /// not an error.
    pub fn remove(&self, id: ModelId) -> Result<()> {
        match fs::remove_dir_all(self.model_dir(id.spec())) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => Err(error.into()),
            _ => Ok(()),
        }
    }

    fn model_dir(&self, spec: &ModelSpec) -> PathBuf {
        self.root.join(spec.install_dir_name())
    }

    /// Removes temporary files left behind by an install that was killed
    /// before it could clean up after itself.
    fn remove_leftovers(&self) -> Result<()> {
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            if !entry.file_name().to_string_lossy().starts_with(TEMP_PREFIX) {
                continue;
            }
            let path = entry.path();
            let removal = if entry.file_type()?.is_dir() {
                fs::remove_dir_all(&path)
            } else {
                fs::remove_file(&path)
            };
            if let Err(error) = removal {
                log::warn!("could not remove leftover {}: {error}", path.display());
            }
        }
        Ok(())
    }
}

/// Extracts the files the recognizer needs from a `.tar.bz2` model archive
/// into `destination`, flattening the archive's folder structure.
fn extract_model(archive: impl Read, destination: &Path, cancel: &AtomicBool) -> Result<()> {
    let mut tar = tar::Archive::new(BzDecoder::new(BufReader::new(archive)));
    for entry in tar.entries()? {
        let mut entry = entry?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path = entry.path()?;
        let Some(name) = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| {
                REQUIRED_FILES
                    .into_iter()
                    .find(|required| *required == name)
            })
        else {
            continue;
        };
        let mut output = File::create(destination.join(name))?;
        copy_cancellable(&mut entry, &mut output, cancel)?;
        output.sync_all()?;
    }

    for name in REQUIRED_FILES {
        if !destination.join(name).is_file() {
            return Err(Error::IncompleteArchive(name));
        }
    }
    Ok(())
}

fn copy_cancellable(
    reader: &mut impl Read,
    writer: &mut impl Write,
    cancel: &AtomicBool,
) -> Result<()> {
    let mut buffer = vec![0; 256 * 1024];
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(Error::Cancelled);
        }
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            return Ok(());
        }
        writer.write_all(&buffer[..read])?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bzip2::Compression;
    use bzip2::write::BzEncoder;

    fn archive_with(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut builder = tar::Builder::new(BzEncoder::new(Vec::new(), Compression::fast()));
        for (path, contents) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(contents.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            builder.append_data(&mut header, path, *contents).unwrap();
        }
        builder.into_inner().unwrap().finish().unwrap()
    }

    #[test]
    fn extracts_only_required_files_flattened() {
        let archive = archive_with(&[
            ("model/encoder.int8.onnx", b"encoder"),
            ("model/decoder.int8.onnx", b"decoder"),
            ("model/joiner.int8.onnx", b"joiner"),
            ("model/tokens.txt", b"tokens"),
            ("model/test_wavs/en.wav", b"audio"),
        ]);
        let destination = tempfile::tempdir().unwrap();

        extract_model(
            archive.as_slice(),
            destination.path(),
            &AtomicBool::new(false),
        )
        .unwrap();

        assert!(ModelFiles::in_dir(destination.path()).all_exist());
        assert_eq!(
            fs::read(destination.path().join(ENCODER)).unwrap(),
            b"encoder"
        );
        assert!(!destination.path().join("en.wav").exists());
    }

    #[test]
    fn rejects_archive_missing_a_required_file() {
        let archive = archive_with(&[("model/encoder.int8.onnx", b"encoder")]);
        let destination = tempfile::tempdir().unwrap();

        let result = extract_model(
            archive.as_slice(),
            destination.path(),
            &AtomicBool::new(false),
        );

        assert!(matches!(result, Err(Error::IncompleteArchive(_))));
    }

    #[test]
    fn stops_extracting_when_cancelled() {
        let archive = archive_with(&[("model/encoder.int8.onnx", b"encoder")]);
        let destination = tempfile::tempdir().unwrap();

        let result = extract_model(
            archive.as_slice(),
            destination.path(),
            &AtomicBool::new(true),
        );

        assert!(matches!(result, Err(Error::Cancelled)));
    }

    #[test]
    fn reports_missing_model_as_not_installed() {
        let root = tempfile::tempdir().unwrap();
        let store = ModelStore::new(root.path());

        assert!(!store.is_installed(ModelId::NemotronBalanced));
        store.remove(ModelId::NemotronBalanced).unwrap();
    }

    #[test]
    fn removes_leftovers_from_interrupted_installs() {
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join(".partial-archive.tar.bz2"), b"partial").unwrap();
        fs::create_dir(root.path().join(".partial-staging")).unwrap();
        fs::create_dir(root.path().join("installed-model")).unwrap();

        ModelStore::new(root.path()).remove_leftovers().unwrap();

        let remaining: Vec<_> = fs::read_dir(root.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(remaining, ["installed-model"]);
    }
}
