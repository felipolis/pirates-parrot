//! The speech models the app can download, with where to get them and how
//! to verify them.

use serde::{Deserialize, Serialize};

/// Identifies one of the speech models Pirate's Parrot can run.
///
/// All entries are the same multilingual NVIDIA Nemotron 3.5 ASR Streaming
/// model exported with different chunk sizes: smaller chunks show words
/// sooner, larger chunks are slightly more accurate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelId {
    NemotronFast,
    #[default]
    NemotronBalanced,
    NemotronAccurate,
}

/// Static description of a downloadable model.
#[derive(Debug)]
pub struct ModelSpec {
    pub id: ModelId,
    pub name: &'static str,
    pub description: &'static str,
    /// Audio processed per decoding step, which bounds how fast words appear.
    pub chunk_ms: u32,
    pub archive_url: &'static str,
    /// Lowercase hex SHA-256 of the archive, as published by the release.
    pub archive_sha256: &'static str,
    pub archive_size: u64,
}

/// URL of the sherpa-onnx export of Nemotron 3.5 for a given chunk size.
macro_rules! nemotron_archive_url {
    ($chunk:literal) => {
        concat!(
            "https://github.com/k2-fsa/sherpa-onnx/releases/download/asr-models/",
            "sherpa-onnx-nemotron-3.5-asr-streaming-0.6b-",
            $chunk,
            "-int8-2026-06-11.tar.bz2"
        )
    };
}

static NEMOTRON_FAST: ModelSpec = ModelSpec {
    id: ModelId::NemotronFast,
    name: "Fast",
    description: "Words appear almost instantly. Slightly less accurate.",
    chunk_ms: 160,
    archive_url: nemotron_archive_url!("160ms"),
    archive_sha256: "a81909a1780d84cff16d73c15e13e67d9d81d8839faf14870d507d8499f7a61a",
    archive_size: 475_273_363,
};

static NEMOTRON_BALANCED: ModelSpec = ModelSpec {
    id: ModelId::NemotronBalanced,
    name: "Balanced",
    description: "Recommended. Responsive and accurate.",
    chunk_ms: 560,
    archive_url: nemotron_archive_url!("560ms"),
    archive_sha256: "c6bf5e0df765f9d5b43bc9e0536d4b4b3e7d40bdf5ecf13e45f134c51c05ae3a",
    archive_size: 475_271_763,
};

static NEMOTRON_ACCURATE: ModelSpec = ModelSpec {
    id: ModelId::NemotronAccurate,
    name: "Accurate",
    description: "Highest accuracy. Words appear about a second later.",
    chunk_ms: 1120,
    archive_url: nemotron_archive_url!("1120ms"),
    archive_sha256: "adbdd5e9fef87300c37cebfcfc4f1ebe56845c860c8a760af0a1dd65ce9beed3",
    archive_size: 475_276_334,
};

impl ModelId {
    /// Every model, from lowest latency to highest accuracy.
    pub const ALL: &[Self] = &[
        Self::NemotronFast,
        Self::NemotronBalanced,
        Self::NemotronAccurate,
    ];

    pub fn spec(self) -> &'static ModelSpec {
        match self {
            Self::NemotronFast => &NEMOTRON_FAST,
            Self::NemotronBalanced => &NEMOTRON_BALANCED,
            Self::NemotronAccurate => &NEMOTRON_ACCURATE,
        }
    }
}

impl ModelSpec {
    /// Name of the archive's top-level folder, which embeds the release date.
    /// Installing under this name lets a future release coexist with the
    /// current one instead of silently replacing it.
    pub(crate) fn install_dir_name(&self) -> &'static str {
        let file_name = self
            .archive_url
            .rsplit_once('/')
            .map_or(self.archive_url, |(_, name)| name);
        file_name.strip_suffix(".tar.bz2").unwrap_or(file_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_spec_matches_its_id() {
        for &id in ModelId::ALL {
            assert_eq!(id.spec().id, id);
        }
    }

    #[test]
    fn install_dir_is_the_archive_stem() {
        assert_eq!(
            ModelId::NemotronBalanced.spec().install_dir_name(),
            "sherpa-onnx-nemotron-3.5-asr-streaming-0.6b-560ms-int8-2026-06-11"
        );
    }

    #[test]
    fn checksums_are_lowercase_sha256_hex() {
        for &id in ModelId::ALL {
            let checksum = id.spec().archive_sha256;
            assert_eq!(checksum.len(), 64);
            assert!(
                checksum
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            );
        }
    }
}
