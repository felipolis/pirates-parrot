//! Small JSON files in the app's config folder. There is no database: user
//! preferences and the widget position are the only state that survives a
//! restart.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::AppError;

/// A value mirrored to a JSON file, written atomically on every change.
#[derive(Debug)]
pub struct JsonFile<T> {
    path: PathBuf,
    value: Mutex<T>,
}

impl<T> JsonFile<T>
where
    T: Serialize + DeserializeOwned + Default + Clone,
{
    /// Loads the file, falling back to defaults when it does not exist yet.
    /// A file that cannot be parsed is set aside as `<name>.invalid` rather
    /// than overwritten, so a hand-edited mistake is never lost silently.
    pub fn load(path: PathBuf) -> Self {
        let value = match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|error| {
                log::warn!("{} is invalid ({error}); using defaults", path.display());
                set_aside(&path);
                T::default()
            }),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => T::default(),
            Err(error) => {
                log::warn!(
                    "could not read {} ({error}); using defaults",
                    path.display()
                );
                T::default()
            }
        };
        Self {
            path,
            value: Mutex::new(value),
        }
    }

    pub fn get(&self) -> T {
        self.lock().clone()
    }

    /// Replaces the value. Memory is only updated once the file is written.
    pub fn set(&self, value: T) -> Result<(), AppError> {
        let mut current = self.lock();
        write_atomically(&self.path, &serde_json::to_vec_pretty(&value)?)?;
        *current = value;
        Ok(())
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, T> {
        // The guarded value is always replaced whole, so it is never left
        // half-updated by a panicking writer.
        self.value
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// Writes through a temporary file in the same folder and renames it over the
/// target, so a crash never leaves a truncated file behind.
fn write_atomically(path: &Path, contents: &[u8]) -> Result<(), AppError> {
    let folder = path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path has no parent folder",
        )
    })?;
    fs::create_dir_all(folder)?;
    let mut file = tempfile::NamedTempFile::new_in(folder)?;
    file.write_all(contents)?;
    file.as_file().sync_all()?;
    file.persist(path).map_err(|error| error.error)?;
    Ok(())
}

fn set_aside(path: &Path) {
    let backup = path.with_extension("invalid");
    if let Err(error) = fs::rename(path, &backup) {
        log::warn!("could not move {} aside: {error}", path.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
    struct Sample {
        name: String,
    }

    #[test]
    fn missing_file_yields_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let file = JsonFile::<Sample>::load(dir.path().join("sample.json"));

        assert_eq!(file.get(), Sample::default());
    }

    #[test]
    fn saved_value_survives_reload() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("sample.json");
        let saved = Sample {
            name: "Captain".to_owned(),
        };

        JsonFile::<Sample>::load(path.clone())
            .set(saved.clone())
            .unwrap();

        assert_eq!(JsonFile::<Sample>::load(path).get(), saved);
    }

    #[test]
    fn invalid_file_is_set_aside() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sample.json");
        fs::write(&path, "{ not json").unwrap();

        let file = JsonFile::<Sample>::load(path.clone());

        assert_eq!(file.get(), Sample::default());
        assert!(!path.exists());
        assert!(dir.path().join("sample.invalid").exists());
    }
}
