use anyhow::Context;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir};
use std::path::{Path, PathBuf};

/// Extend [Path] with [anyhow] methods
pub trait PathAnyhow {
    /// Wrap [Path::to_str], providing the path as error context
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;

    /// Wrap [Path::parent], providing the path as error context
    fn parent_anyhow(&self) -> anyhow::Result<&Path>;

    /// Wrap [Path::file_name], providing the path as error context
    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::strip_prefix], providing the path and `base` as error context
    fn strip_prefix_anyhow<P>(&self, base: P) -> anyhow::Result<&Path>
    where
        P: AsRef<Path>;

    /// Wrap [Path::file_stem], providing the path as error context
    fn file_stem_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::extension], providing the path as error context
    fn extension_anyhow(&self) -> anyhow::Result<&OsStr>;

    /// Wrap [Path::metadata], providing the path as error context
    fn metadata_anyhow(&self) -> anyhow::Result<Metadata>;

    /// Wrap [Path::symlink_metadata], providing the path as error context
    fn symlink_metadata_anyhow(&self) -> anyhow::Result<Metadata>;

    /// Wrap [Path::canonicalize], providing the path as error context
    fn canonicalize_anyhow(&self) -> anyhow::Result<PathBuf>;

    /// Wrap [Path::read_link], providing the path as error context
    fn read_link_anyhow(&self) -> anyhow::Result<PathBuf>;

    /// Wrap [Path::read_dir], providing the path as error context
    fn read_dir_anyhow(&self) -> anyhow::Result<ReadDir>;
}

macro_rules! wrap_nullary_option_method {
    ( $method:ident, $cb:expr, $ret:ty, $errordesc:expr ) => {
        fn $method(&self) -> anyhow::Result<$ret> {
            let p = self.as_ref();
            $cb(p)
                .ok_or_else(|| anyhow::Error::msg($errordesc))
                .with_context(|| format!("while processing path {:?}", p.display()))
        }
    };
}

macro_rules! wrap_nullary_result_method {
    ( $method:ident, $cb:expr, $ret:ty ) => {
        fn $method(&self) -> anyhow::Result<$ret> {
            $cb(self).with_context(|| format!("while processing path {:?}", self.display()))
        }
    };
}

impl PathAnyhow for Path {
    wrap_nullary_option_method!(to_str_anyhow, Path::to_str, &str, "invalid UTF8");

    wrap_nullary_option_method!(
        parent_anyhow,
        Path::parent,
        &Path,
        "expected parent directory"
    );

    wrap_nullary_option_method!(
        file_name_anyhow,
        Path::file_name,
        &OsStr,
        "missing expected filename"
    );

    fn strip_prefix_anyhow<Q>(&self, base: Q) -> anyhow::Result<&Path>
    where
        Q: AsRef<Path>,
    {
        let bref = base.as_ref();
        self.strip_prefix(bref)
            .with_context(|| format!("with prefix {:?}", bref.display()))
            .with_context(|| format!("while processing path {:?}", self.display()))
    }

    wrap_nullary_option_method!(
        file_stem_anyhow,
        Path::file_stem,
        &OsStr,
        "missing expected filename"
    );

    wrap_nullary_option_method!(
        extension_anyhow,
        Path::extension,
        &OsStr,
        "missing expected extension"
    );

    wrap_nullary_result_method!(metadata_anyhow, Path::metadata, Metadata);
    wrap_nullary_result_method!(symlink_metadata_anyhow, Path::symlink_metadata, Metadata);
    wrap_nullary_result_method!(canonicalize_anyhow, Path::canonicalize, PathBuf);
    wrap_nullary_result_method!(read_link_anyhow, Path::read_link, PathBuf);
    wrap_nullary_result_method!(read_dir_anyhow, Path::read_dir, ReadDir);
}

#[cfg(test)]
mod tests;
