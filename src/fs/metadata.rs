use anyhow::Context;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::SystemTime;

/// Wraps [std::fs::Metadata] to provide the path as error context
#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct Metadata {
    md: std::fs::Metadata,
    path: PathBuf,
}

impl Metadata {
    /// Extend [std::fs::Metadata::modified] to provide the path as error context
    pub fn modified(&self) -> anyhow::Result<SystemTime> {
        self.md
            .modified()
            .with_context(|| format!("while processing path {:?}", self.path.display()))
    }

    /// Extend [std::fs::Metadata::accessed] to provide the path as error context
    pub fn accessed(&self) -> anyhow::Result<SystemTime> {
        self.md
            .accessed()
            .with_context(|| format!("while processing path {:?}", self.path.display()))
    }

    /// Extend [std::fs::Metadata::created] to provide the path as error context
    pub fn created(&self) -> anyhow::Result<SystemTime> {
        self.md
            .created()
            .with_context(|| format!("while processing path {:?}", self.path.display()))
    }
}

impl Deref for Metadata {
    type Target = std::fs::Metadata;

    fn deref(&self) -> &Self::Target {
        &self.md
    }
}
