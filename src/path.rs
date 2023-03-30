use anyhow::Context;
use std::ffi::OsStr;
use std::path::Path;

pub trait PathAnyhow {
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;
    fn parent_anyhow(&self) -> anyhow::Result<&Path>;
    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr>;
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
}

#[cfg(test)]
mod tests;
