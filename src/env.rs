use anyhow::Context;
use std::ffi::OsStr;

pub fn var<K>(key: K) -> anyhow::Result<String>
where
    K: AsRef<OsStr>,
{
    let os = key.as_ref();
    std::env::var(os).with_context(|| format!("environment variable {:?}", os.to_string_lossy()))
}

#[cfg(test)]
mod tests;
