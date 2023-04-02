//! Wrappers for [std::fs::ReadDir] and [std::fs::DirEntry] which provide paths in error contexts

use anyhow::Context;
use std::fs::FileType;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Wraps [std::fs::ReadDir] to provide the directory as error context
#[derive(Debug)]
pub struct ReadDir {
    rd: std::fs::ReadDir,
    path: PathBuf,
}

impl ReadDir {
    pub(crate) fn from_path(p: &Path) -> std::io::Result<Self> {
        p.read_dir().map(|rd| ReadDir {
            rd,
            path: p.to_path_buf(),
        })
    }
}

impl Iterator for ReadDir {
    type Item = anyhow::Result<DirEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rd.next().map(|stditem| {
            stditem
                .map(|de| DirEntry { de })
                .with_context(|| format!("while reading directory {:?}", self.path.display()))
        })
    }
}

/// Wraps [std::fs::DirEntry] to provide the path as error context
#[derive(Debug)]
pub struct DirEntry {
    de: std::fs::DirEntry,
}

impl DirEntry {
    /// Wrap [std::fs::DirEntry::metadata] providing the path in the error context
    pub fn metadata(&self) -> anyhow::Result<Metadata> {
        self.de
            .metadata()
            .map(|md| Metadata {
                md,
                path: self.path(),
            })
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

/// Wraps [std::fs::DirEntry] to provide the path as error context
#[derive(Debug)]
pub struct Metadata {
    md: std::fs::Metadata,
    path: PathBuf,
}

impl Metadata {
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

#[cfg(test)]
mod tests;
