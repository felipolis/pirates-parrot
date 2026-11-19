//! Speech model catalog, download and installation.

mod catalog;
mod download;
mod store;

pub use catalog::{ModelId, ModelSpec};
pub use store::{InstallProgress, ModelFiles, ModelStore};
