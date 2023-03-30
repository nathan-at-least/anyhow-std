use anyhow::Context;
use std::ffi::OsStr;
use std::fs::{Metadata, ReadDir};
use std::path::{Path, PathBuf};

pub trait PathAnyhow {
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;
    fn parent_anyhow(&self) -> anyhow::Result<&Path>;
    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr>;
    fn strip_prefix_anyhow<P>(&self, base: P) -> anyhow::Result<&Path>
    where
        P: AsRef<Path>;
    fn file_stem_anyhow(&self) -> anyhow::Result<&OsStr>;
    fn extension_anyhow(&self) -> anyhow::Result<&OsStr>;
    fn metadata_anyhow(&self) -> anyhow::Result<Metadata>;
    fn symlink_metadata_anyhow(&self) -> anyhow::Result<Metadata>;
    fn canonicalize_anyhow(&self) -> anyhow::Result<PathBuf>;
    fn read_link_anyhow(&self) -> anyhow::Result<PathBuf>;
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
            let p = self.as_ref();
            $cb(p).with_context(|| format!("while processing path {:?}", p.display()))
        }
    };
}

impl<P> PathAnyhow for P
where
    P: AsRef<Path>,
{
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
        let p = self.as_ref();
        let bref = base.as_ref();
        p.strip_prefix(bref)
            .with_context(|| format!("with prefix {:?}", bref.display()))
            .with_context(|| format!("while processing path {:?}", p.display()))
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
