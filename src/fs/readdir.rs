use crate::fs::DirEntry;
use anyhow::Context;
use std::path::{Path, PathBuf};

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
        wrap_read_dir_item(&self.path, self.rd.next())
    }
}

fn wrap_read_dir_item(
    path: &Path,
    item: Option<std::io::Result<std::fs::DirEntry>>,
) -> Option<anyhow::Result<DirEntry>> {
    item.map(|stditem| {
        stditem
            .map(DirEntry::wrap)
            .with_context(|| format!("while reading directory {:?}", path.display()))
    })
}

#[cfg(test)]
mod tests;
