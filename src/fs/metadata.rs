use anyhow::Context;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::SystemTime;

/// Wraps [std::fs::DirEntry] to provide the path as error context
#[derive(Debug)]
pub struct Metadata {
    md: std::fs::Metadata,
    path: PathBuf,
}

impl Metadata {
    pub(crate) fn wrap(md: std::fs::Metadata, path: PathBuf) -> Self {
        Metadata { md, path }
    }

    /// Wrap [std::fs::Metadata::modified] to provide the path as error context
    pub fn modified(&self) -> anyhow::Result<SystemTime> {
        self.md
            .modified()
            .with_context(|| format!("while processing path {:?}", self.path.display()))
    }

    /// Wrap [std::fs::Metadata::accessed] to provide the path as error context
    pub fn accessed(&self) -> anyhow::Result<SystemTime> {
        self.md
            .accessed()
            .with_context(|| format!("while processing path {:?}", self.path.display()))
    }

    /// Wrap [std::fs::Metadata::created] to provide the path as error context
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
