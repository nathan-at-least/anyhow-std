use crate::fs::Metadata;
use crate::fs::ReadDir;
use anyhow::Context;
use std::ffi::OsStr;
use std::fs::{File, Permissions};
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

    // Wrappers for std::fs:

    /// Wrap [std::fs::copy] from `self` to `to`, providing `self` and `to` as error context
    fn copy_anyhow<P>(&self, to: P) -> anyhow::Result<u64>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::create_dir], providing the path as error context
    fn create_dir_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::create_dir_all], providing the path as error context
    fn create_dir_all_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::hard_link], providing `self` and `link` as error context
    fn hard_link_anyhow<P>(&self, link: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::read], providing the path as error context
    fn read_anyhow(&self) -> anyhow::Result<Vec<u8>>;

    /// Wrap [std::fs::read_to_string], providing the path as error context
    fn read_to_string_anyhow(&self) -> anyhow::Result<String>;

    /// Wrap [std::fs::remove_dir], providing the path as error context
    fn remove_dir_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::remove_dir_all], providing the path as error context
    fn remove_dir_all_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::remove_file], providing the path as error context
    fn remove_file_anyhow(&self) -> anyhow::Result<()>;

    /// Wrap [std::fs::rename], providing `self` and `to` as error context
    fn rename_anyhow<P>(&self, to: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>;

    /// Wrap [std::fs::set_permissions], providing the path as error context
    fn set_permissions_anyhow(&self, perm: Permissions) -> anyhow::Result<()>;

    /// Toggle read-only permission for the path
    ///
    /// This method factors out the complexity of retrieving [std::fs::Permissions], modifying
    /// them, and then setting them.
    fn set_readonly_anyhow(&self, readonly: bool) -> anyhow::Result<()>;

    /// Wrap [std::fs::write], providing the path as error context
    fn write_anyhow<C>(&self, contents: C) -> anyhow::Result<()>
    where
        C: AsRef<[u8]>;

    /// Wrap [std::env::set_current_dir], providing the path as error context
    fn set_to_current_dir_anyhow(&self) -> anyhow::Result<()>;

    // File APIs:
    /// Open a [File] in read-only mode wrapping [File::open]
    fn open_file_anyhow(&self) -> anyhow::Result<File>;

    /// Open a [File] in write-only mode wrapping [File::create]
    fn create_file_anyhow(&self) -> anyhow::Result<File>;
}

macro_rules! wrap_method {
    ( $method:ident, $cb:expr, $ret:ty, None: $errordesc:expr ) => {
        fn $method(&self) -> anyhow::Result<$ret> {
            let p = self.as_ref();
            $cb(p)
                .ok_or_else(|| anyhow::Error::msg($errordesc))
                .with_context(|| format!("while processing path {:?}", p.display()))
        }
    };

    ( $method:ident, $cb:expr, $ret:ty ) => {
        #[allow(clippy::redundant_closure_call)]
        fn $method(&self) -> anyhow::Result<$ret> {
            $cb(self).with_context(|| format!("while processing path {:?}", self.display()))
        }
    };

    ( $method:ident, $cb:expr, AsRefPath: $arg:ident, $ret:ty ) => {
        fn $method<Q>(&self, $arg: Q) -> anyhow::Result<$ret>
        where
            Q: AsRef<Path>,
        {
            let argref = $arg.as_ref();
            $cb(self, argref)
                .with_context(|| format!("with {} {:?}", stringify!($arg), argref.display()))
                .with_context(|| format!("while processing path {:?}", self.display()))
        }
    };
}

impl PathAnyhow for Path {
    wrap_method!(to_str_anyhow, Path::to_str, &str, None: "invalid UTF8");

    wrap_method!(
        parent_anyhow,
        Path::parent,
        &Path,
        None: "expected parent directory"
    );

    wrap_method!(
        file_name_anyhow,
        Path::file_name,
        &OsStr,
        None: "missing expected filename"
    );

    wrap_method!(
        strip_prefix_anyhow,
        Path::strip_prefix,
        AsRefPath: prefix,
        &Path
    );

    wrap_method!(
        file_stem_anyhow,
        Path::file_stem,
        &OsStr,
        None: "missing expected filename"
    );

    wrap_method!(
        extension_anyhow,
        Path::extension,
        &OsStr,
        None: "missing expected extension"
    );

    wrap_method!(
        metadata_anyhow,
        |p: &Path| p.metadata().map(|md| Metadata::from((md, p.to_path_buf()))),
        Metadata
    );
    wrap_method!(
        symlink_metadata_anyhow,
        |p: &Path| p
            .symlink_metadata()
            .map(|md| Metadata::from((md, p.to_path_buf()))),
        Metadata
    );
    wrap_method!(canonicalize_anyhow, Path::canonicalize, PathBuf);
    wrap_method!(read_link_anyhow, Path::read_link, PathBuf);
    wrap_method!(read_dir_anyhow, ReadDir::from_path, ReadDir);
    wrap_method!(copy_anyhow, std::fs::copy, AsRefPath: copy_to, u64);
    wrap_method!(create_dir_anyhow, std::fs::create_dir, ());
    wrap_method!(create_dir_all_anyhow, std::fs::create_dir_all, ());
    wrap_method!(hard_link_anyhow, std::fs::hard_link, AsRefPath: link_to, ());
    wrap_method!(read_anyhow, std::fs::read, Vec<u8>);
    wrap_method!(read_to_string_anyhow, std::fs::read_to_string, String);
    wrap_method!(remove_dir_anyhow, std::fs::remove_dir, ());
    wrap_method!(remove_dir_all_anyhow, std::fs::remove_dir_all, ());
    wrap_method!(remove_file_anyhow, std::fs::remove_file, ());
    wrap_method!(rename_anyhow, std::fs::rename, AsRefPath: rename_to, ());

    fn set_permissions_anyhow(&self, perms: Permissions) -> anyhow::Result<()> {
        std::fs::set_permissions(self, perms.clone())
            .with_context(|| format!("with permissions {:?}", perms))
            .with_context(|| format!("while processing path {:?}", self.display()))
    }

    fn set_readonly_anyhow(&self, readonly: bool) -> anyhow::Result<()> {
        let mut perms = self.metadata_anyhow()?.permissions();
        perms.set_readonly(readonly);
        self.set_permissions_anyhow(perms)
    }

    fn write_anyhow<C>(&self, contents: C) -> anyhow::Result<()>
    where
        C: AsRef<[u8]>,
    {
        std::fs::write(self, contents)
            .with_context(|| format!("while writing to {:?}", self.display()))
    }

    wrap_method!(set_to_current_dir_anyhow, std::env::set_current_dir, ());
    wrap_method!(open_file_anyhow, File::open, File);
    wrap_method!(create_file_anyhow, File::create, File);
}

#[cfg(test)]
mod tests;
