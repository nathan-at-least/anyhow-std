use anyhow::Context;
use std::ffi::OsStr;
use std::path::Path;

pub trait PathAnyhow {
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;
    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr>;
}

impl<P> PathAnyhow for P
where
    P: AsRef<Path>,
{
    fn to_str_anyhow(&self) -> anyhow::Result<&str> {
        let p = self.as_ref();
        p.to_str()
            .ok_or_else(|| anyhow::Error::msg("invalid UTF8"))
            .with_context(|| format!("while processing path {:?}", p.display()))
    }

    fn file_name_anyhow(&self) -> anyhow::Result<&OsStr> {
        let p = self.as_ref();
        p.file_name()
            .ok_or_else(|| anyhow::Error::msg("missing expected filename"))
            .with_context(|| format!("while processing path {:?}", p.display()))
    }
}

#[cfg(test)]
mod tests;
