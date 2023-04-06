use crate::fs::Metadata;
use anyhow::Context;
use std::fs::FileType;
use std::ops::Deref;

/// Wraps [std::fs::DirEntry] to provide the path as error context
#[derive(Debug)]
pub struct DirEntry {
    de: std::fs::DirEntry,
}

impl DirEntry {
    pub(crate) fn wrap(de: std::fs::DirEntry) -> Self {
        DirEntry { de }
    }

    /// Wrap [std::fs::DirEntry::metadata] providing the path in the error context
    pub fn metadata(&self) -> anyhow::Result<Metadata> {
        self.de
            .metadata()
            .map(|md| Metadata::wrap(md, self.path()))
            .with_context(|| format!("while processing path {:?}", self.path().display()))
    }

    /// Wrap [std::fs::DirEntry::file_type] providing the path in the error context
    pub fn file_type(&self) -> anyhow::Result<FileType> {
        self.de
            .file_type()
            .with_context(|| format!("while processing path {:?}", self.path().display()))
    }
}

impl Deref for DirEntry {
    type Target = std::fs::DirEntry;

    fn deref(&self) -> &Self::Target {
        &self.de
    }
}
