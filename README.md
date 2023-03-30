# `anyhow-std`

This crate wraps certain [std] functionality to produce [anyhow::Result]s
providing better error messages w/ context.

## Extension Traits

A consistent pattern is used to extend [std] types:

- An extension trait is provided named `<std type>Anyhow`, for example: [OsStrAnyhow]
- An impl is provided for the target type, for example `impl OsStrAnyhow for OsStr { … }`
- For a subset of methods of the target type, extension trait methods are provided named `<method name>_anyhow`. These methods always return [anyhow::Result] types, for example: [OsStrAnyhow::to_str_anyhow].

### `…_anyhow` Methods

These methods convert `Option<T>` or `Result<T, E>` return types of
the [std] method into `anyhow::Result<T>`, where the [anyhow::Error]
has context added specific to the [std] type. Often a `None` return type
isn't necessarily an "error" per se, so callers choose whether to use the
[std] target method or the `…_anyhow` method based on whether or not
to treat the result as an error.

#### Example: Expecting a Path Extension

Suppose we are processing a user-provided path and we _expect_ it to
have an extension that is valid UTF8:

```
use std::path::Path;
use anyhow_std::{PathAnyhow, OsStrAnyhow};

fn process_user_path(path: &Path) -> anyhow::Result<()> {
  let extos = path.extension_anyhow()?;
  let ext = extos.to_str_anyhow()?;
  process_user_path_with_extension(path, ext)?;
  Ok(())
}

fn process_user_path_with_extension(path: &Path, extension: &str) -> anyhow::Result<()> {
  todo!("implement path processing of {path:?} for the extension {extension:?}")
}
```
