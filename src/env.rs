//! Wrappers for [mod@std::env]
use anyhow::{anyhow, Context};
use std::ffi::{OsStr, OsString};

/// Wrap [std::env::var] providing `key` in error context
pub fn var<K>(key: K) -> anyhow::Result<String>
where
    K: AsRef<OsStr>,
{
    let os = key.as_ref();
    std::env::var(os).with_context(|| format!("environment variable {:?}", os.to_string_lossy()))
}

/// Wrap [std::env::var_os], converting `None` to an error with `key` in error context
pub fn var_os<K>(key: K) -> anyhow::Result<OsString>
where
    K: AsRef<OsStr>,
{
    let os = key.as_ref();
    var_os_without_context(os)
        .with_context(|| format!("environment variable {:?}", os.to_string_lossy()))
}

fn var_os_without_context<K>(key: K) -> anyhow::Result<OsString>
where
    K: AsRef<OsStr>,
{
    let lossy_cow = key.as_ref().to_string_lossy();
    let lossy_str = lossy_cow.as_ref();
    for c in ['=', '\0'] {
        if lossy_str.contains(c) {
            return Err(anyhow!("environment variable contains {:?}", c));
        }
    }
    std::env::var_os(key).ok_or_else(|| anyhow!("environment variable not found"))
}

#[cfg(test)]
mod tests;
