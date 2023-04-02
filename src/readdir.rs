//! Wrappers for [std::fs::ReadDir] and [std::fs::DirEntry] which provide paths in error contexts

use std::path::Path;

/// Wraps [std::fs::ReadDir] to provide the directory as error context
pub struct ReadDir(std::fs::ReadDir);

impl ReadDir {
    pub(crate) fn from_path(p: &Path) -> std::io::Result<Self> {
        p.read_dir().map(ReadDir)
    }
}
