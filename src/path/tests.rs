use crate::PathAnyhow;
use indoc::indoc;
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
    let error = format!("{:?}", path.to_str_anyhow().err().unwrap());
    assert_eq!(
        error,
        indoc! { r#"
            while processing path "\x81\xFF"

            Caused by:
                invalid UTF8
        "#
        }
        .trim_end()
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
    let error = format!("{:?}", path.file_name_anyhow().err().unwrap());
    assert_eq!(
        error,
        indoc! { r#"
            while processing path "/foo/.."

            Caused by:
                missing expected filename
        "#
        }
        .trim_end()
    );
    Ok(())
}
