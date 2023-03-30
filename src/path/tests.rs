use crate::PathAnyhow;
use std::path::Path;

#[test]
fn to_str_utf8() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("/foo/bar.txt", path.to_str_anyhow()?);
    Ok(())
}

#[cfg(target_family = "unix")]
#[test]
fn to_str_invalid_utf8() -> anyhow::Result<()> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let path = Path::new(OsStr::from_bytes(b"\x81\xff"));
    assert_error_desc_eq(
        path.to_str_anyhow(),
        r#"while processing path "\x81\xFF": invalid UTF8"#,
    );
    Ok(())
}

#[test]
fn parent_non_root() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    let expected = Path::new("/foo/");
    assert_eq!(expected, path.parent_anyhow()?);
    Ok(())
}

#[test]
fn parent_root() -> anyhow::Result<()> {
    let path = Path::new("/");
    assert_error_desc_eq(
        path.parent_anyhow(),
        r#"while processing path "/": expected parent directory"#,
    );
    Ok(())
}

#[test]
fn file_name_present() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar.txt");
    assert_eq!("bar.txt", path.file_name_anyhow()?);
    Ok(())
}

#[test]
fn file_name_missing() -> anyhow::Result<()> {
    let path = Path::new("/foo/..");
    assert_error_desc_eq(
        path.file_name_anyhow(),
        r#"while processing path "/foo/..": missing expected filename"#,
    );
    Ok(())
}

#[test]
fn strip_prefix_ok() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar/quz.txt");
    let expected = Path::new("bar/quz.txt");
    assert_eq!(expected, path.strip_prefix_anyhow("/foo")?);
    Ok(())
}

#[test]
fn strip_prefix_err() -> anyhow::Result<()> {
    let path = Path::new("/foo/bar/quz.txt");
    assert_error_desc_eq(
        path.strip_prefix_anyhow("/bananas"),
        r#"while processing path "/foo/bar/quz.txt": with prefix "/bananas": prefix not found"#,
    );
    Ok(())
}

fn assert_error_desc_eq<T>(res: anyhow::Result<T>, expected: &str) {
    let error = format!("{:#}", res.err().unwrap());
    assert_eq!(error, expected.trim_end());
}
