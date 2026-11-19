//! Lifecycle of the speech model: which one is active, whether it is
//! installed, downloading, loading or ready to transcribe.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use parrot_core::{InstallProgress, ModelFiles, ModelId, ModelStore, Transcriber};
use serde::Serialize;
use tauri::async_runtime::spawn_blocking;
use tauri::{AppHandle, Manager, State};

use crate::error::AppError;
use crate::events;

/// State of the active model as shown to the user.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "camelCase")]
pub enum EngineStatus {
    NotInstalled,
    Installing { progress: InstallProgress },
    Loading,
    Ready,
    Failed { message: String },
}

/// A model from the catalog, with its local state.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelInfo {
    pub id: ModelId,
    pub name: &'static str,
    pub description: &'static str,
    pub chunk_ms: u32,
    pub download_bytes: u64,
    pub installed: bool,
}

/// Owns the loaded [`Transcriber`] and the background jobs that install and
/// load models.
#[derive(Debug)]
pub struct Engine {
    store: ModelStore,
    inner: Mutex<Inner>,
}

#[derive(Debug)]
struct Inner {
    active: ModelId,
    status: EngineStatus,
    transcriber: Option<Arc<Transcriber>>,
    install: Option<InstallJob>,
    /// Bumped whenever the active model changes, so a load or install that
    /// finishes after the user switched models does not clobber the new state.
    generation: u64,
}

#[derive(Debug)]
struct InstallJob {
    model: ModelId,
    cancel: Arc<AtomicBool>,
    /// Last progress reported, shown again if the user returns to this model.
    progress: InstallProgress,
}

impl Engine {
    pub fn new(store: ModelStore, active: ModelId) -> Self {
        Self {
            store,
            inner: Mutex::new(Inner {
                active,
                status: EngineStatus::NotInstalled,
                transcriber: None,
                install: None,
                generation: 0,
            }),
        }
    }

    pub fn status(&self) -> EngineStatus {
        self.lock().status.clone()
    }

    /// The loaded model, if it is ready.
    pub fn transcriber(&self) -> Option<Arc<Transcriber>> {
        self.lock().transcriber.clone()
    }

    pub fn models(&self) -> Vec<ModelInfo> {
        ModelId::ALL
            .iter()
            .map(|&id| {
                let spec = id.spec();
                ModelInfo {
                    id,
                    name: spec.name,
                    description: spec.description,
                    chunk_ms: spec.chunk_ms,
                    download_bytes: spec.archive_size,
                    installed: self.store.is_installed(id),
                }
            })
            .collect()
    }

    /// Makes `model` the active one and loads it in the background if it is
    /// installed. A download of a different model is cancelled.
    pub fn activate(&self, app: &AppHandle, model: ModelId) {
        let mut inner = self.lock();
        inner.active = model;
        inner.generation += 1;
        inner.transcriber = None;

        if let Some(job) = &inner.install {
            let resumed = job.model == model;
            // Coming back to a model whose download was just cancelled lets it
            // carry on if it has not stopped yet; if it has, it reports
            // `Cancelled` and the user can start it again.
            job.cancel.store(!resumed, Ordering::Relaxed);
            if resumed {
                // The install reports its progress and loads the model when done.
                let progress = job.progress;
                set_status(app, &mut inner, EngineStatus::Installing { progress });
                return;
            }
        }
        if self.store.is_installed(model) {
            self.start_loading(app, &mut inner);
        } else {
            set_status(app, &mut inner, EngineStatus::NotInstalled);
        }
    }

    /// Downloads and installs the active model, then loads it.
    pub fn install_active(&self, app: &AppHandle) -> Result<(), AppError> {
        let mut inner = self.lock();
        if inner.install.is_some() {
            return Err(AppError::InstallInProgress);
        }
        let model = inner.active;
        let cancel = Arc::new(AtomicBool::new(false));
        let progress = InstallProgress::Downloading {
            downloaded: 0,
            total: model.spec().archive_size,
        };
        inner.install = Some(InstallJob {
            model,
            cancel: Arc::clone(&cancel),
            progress,
        });
        set_status(app, &mut inner, EngineStatus::Installing { progress });
        drop(inner);

        let app = app.clone();
        spawn_blocking(move || {
            let engine = engine_of(&app);
            let result = engine.store.install(model, &cancel, |progress| {
                engine.report_install_progress(&app, model, progress);
            });
            engine.finish_install(&app, model, result);
        });
        Ok(())
    }

    pub fn cancel_install(&self) {
        if let Some(job) = &self.lock().install {
            job.cancel.store(true, Ordering::Relaxed);
        }
    }

    /// Deletes a model that is not in use.
    pub fn remove(&self, model: ModelId) -> Result<(), AppError> {
        let inner = self.lock();
        if inner.active == model || inner.install.as_ref().is_some_and(|job| job.model == model) {
            return Err(AppError::ModelInUse);
        }
        drop(inner);
        Ok(self.store.remove(model)?)
    }

    fn start_loading(&self, app: &AppHandle, inner: &mut Inner) {
        set_status(app, inner, EngineStatus::Loading);
        let generation = inner.generation;
        let model = inner.active;
        let Some(files) = self.store.installed_files(model) else {
            set_status(app, inner, EngineStatus::NotInstalled);
            return;
        };

        let app = app.clone();
        spawn_blocking(move || {
            let loaded = Transcriber::load(&files);
            let engine = engine_of(&app);
            let mut inner = engine.lock();
            if inner.generation != generation {
                return;
            }
            match loaded {
                Ok(transcriber) => {
                    log::info!("speech model {model:?} loaded");
                    inner.transcriber = Some(Arc::new(transcriber));
                    set_status(&app, &mut inner, EngineStatus::Ready);
                }
                Err(error) => {
                    log::error!("could not load speech model {model:?}: {error}");
                    let message = error.to_string();
                    set_status(&app, &mut inner, EngineStatus::Failed { message });
                }
            }
        });
    }

    fn report_install_progress(&self, app: &AppHandle, model: ModelId, progress: InstallProgress) {
        let mut inner = self.lock();
        if let Some(job) = &mut inner.install {
            job.progress = progress;
        }
        if inner.active == model {
            set_status(app, &mut inner, EngineStatus::Installing { progress });
        }
    }

    fn finish_install(
        &self,
        app: &AppHandle,
        model: ModelId,
        result: parrot_core::Result<ModelFiles>,
    ) {
        let mut inner = self.lock();
        inner.install = None;
        let is_active = inner.active == model;

        match result {
            Ok(_) => {
                log::info!("speech model {model:?} installed");
                if is_active {
                    self.start_loading(app, &mut inner);
                }
            }
            Err(parrot_core::Error::Cancelled) => {
                log::info!("installation of {model:?} cancelled");
                if is_active {
                    set_status(app, &mut inner, EngineStatus::NotInstalled);
                }
            }
            Err(error) => {
                log::error!("could not install {model:?}: {error}");
                if is_active {
                    let message = error.to_string();
                    set_status(app, &mut inner, EngineStatus::Failed { message });
                }
            }
        }
    }

    fn lock(&self) -> MutexGuard<'_, Inner> {
        // Every critical section leaves `Inner` consistent before any call
        // that could panic, so a poisoned lock still holds valid state.
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

fn set_status(app: &AppHandle, inner: &mut Inner, status: EngineStatus) {
    if inner.status != status {
        inner.status = status;
        events::engine_status(app, &inner.status);
    }
}

fn engine_of(app: &AppHandle) -> State<'_, Engine> {
    app.state::<Engine>()
}
