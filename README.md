# `anyhow-std`

This crate wraps certain [std] functionality to produce [anyhow::Result]s
providing better error messages w/ context.

## Example: Expecting a UTF8 Path Extension

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

/*
Now if the user provides a path without an extension, they'll get an
error message with more helpful context:
*/

let res = process_user_path(Path::new("/tmp/no-extension"));
assert!(res.is_err());

let error_message = format!("{:#}", res.err().unwrap());
assert_eq!(
    error_message,
    r#"while processing path "/tmp/no-extension": missing expected extension"#
);

/*
Unix systems can have non-UTF8 paths:
*/

#[cfg(target_family = "unix")]
fn demonstrate_non_utf8_extension() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let res = process_user_path(Path::new(OsStr::from_bytes(b"/tmp/non-unicode-extension.wacky-\xf3-extension")));
    assert!(res.is_err());

    let error_message = format!("{:#}", res.err().unwrap());
    assert_eq!(
        error_message,
        r#"while processing os string "wacky-�-extension": not valid utf8"#,
    );
}

#[cfg(target_family = "unix")]
demonstrate_non_utf8_extension();
```

## Extension Traits

A consistent pattern is used to extend [std] types:

- An extension trait is provided named `<std type>Anyhow`, for example: [OsStrAnyhow]
- An impl is provided for the target type, for example `impl OsStrAnyhow for OsStr { … }`
- For a subset of methods of the target type, extension trait methods are provided named `<method name>_anyhow`. These methods always return [anyhow::Result] types, for example: [`OsStrAnyhow::to_str_anyhow`].

By consistently appending `_anyhow` to wrapped methods, callers can
unambiguously choose when to use these methods versus the [std] methods.

### `…_anyhow` Methods

These methods convert `Option<T>` or `Result<T, E>` return types of
the [std] method into `anyhow::Result<T>`, where the [anyhow::Error]
has context added specific to the [std] type.

For [std] methods that return `Option<T>`, often a `None` return type
isn't necessarily an "error" per se, but the `…_anyhow` methods result
in an error in this case. So these methods should only be used when code
expects `Some` results, and if your code should handle `None` as a
"non-error", it can simply use the [std] method.
