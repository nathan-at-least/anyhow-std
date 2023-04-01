// BUGS: Many tests use unix-specific paths, primarily by assuming "/" exists as a directory.

use crate::{OsStrAnyhow, PathAnyhow};
use std::ffi::OsStr;
use std::path::Path;
use test_case::test_case;

#[test_case("/foo/bar.txt" => Ok("/foo/bar.txt") ; "ok")]
#[cfg(target_family = "unix")]
#[test_case(
    {
        use std::os::unix::ffi::OsStrExt;

        OsStr::from_bytes(b"\x81\xff")
    }
    => err_str(r#"while processing path "\x81\xFF": invalid UTF8"#)
    ; "invalid utf8"
)]
fn to_str<S>(input: &S) -> Result<&str, String>
where
    S: AsRef<OsStr> + ?Sized,
{
    stringify_error(Path::new(input).to_str_anyhow())
}

#[test_case("/foo/bar.txt" => Ok(Path::new("/foo/")); "ok")]
#[test_case("/" => err_str(r#"while processing path "/": expected parent directory"#); "root")]
fn parent(input: &str) -> Result<&Path, String> {
    stringify_error(Path::new(input).parent_anyhow())
}

#[test_case("/foo/bar.txt" => Ok("bar.txt"); "ok")]
#[test_case("/foo/.." => err_str(r#"while processing path "/foo/..": missing expected filename"#); "dot-dot")]
fn file_name(input: &str) -> Result<&str, String> {
    stringify_error(
        Path::new(input)
            .file_name_anyhow()
            .and_then(|s| s.to_str_anyhow()),
    )
}

#[test_case("/foo/bar/quz.txt", "/foo" => Ok("bar/quz.txt"); "ok")]
#[test_case(
    "/foo/bar/quz.txt",
    "/bananas"
    => err_str(
        r#"while processing path "/foo/bar/quz.txt": with prefix "/bananas": prefix not found"#,
    )
    ; "err"
)]
fn strip_prefix<'a>(path: &'a str, prefix: &str) -> Result<&'a str, String> {
    stringify_error(
        Path::new(path)
            .strip_prefix_anyhow(prefix)
            .and_then(|p| p.to_str_anyhow()),
    )
}

#[test_case("/foo/bar.txt" => Ok("bar"); "ok present")]
#[test_case("/foo/bar" => Ok("bar"); "ok absent")]
#[test_case(
    "/foo/.."
    => err_str(
        r#"while processing path "/foo/..": missing expected filename"#,
    );
    "err missing filename"
)]
fn file_stem(input: &str) -> Result<&str, String> {
    stringify_error(
        Path::new(input)
            .file_stem_anyhow()
            .and_then(|p| p.to_str_anyhow()),
    )
}

#[test_case("/foo/bar.txt" => Ok("txt"); "ok")]
#[test_case(
    "/foo/.."
    => err_str(
        r#"while processing path "/foo/..": missing expected extension"#,
    );
    "err missing filename"
)]
#[test_case(
    "/foo/bar"
    => err_str(
        r#"while processing path "/foo/bar": missing expected extension"#,
    );
    "err missing extension"
)]
#[test_case(
    "/foo/.bar"
    => err_str(
        r#"while processing path "/foo/.bar": missing expected extension"#,
    );
    "err dotfile missing extension"
)]
fn extension(input: &str) -> Result<&str, String> {
    stringify_error(
        Path::new(input)
            .extension_anyhow()
            .and_then(|p| p.to_str_anyhow()),
    )
}

#[test_case("/" => Ok(()); "ok root")]
#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    "err missing"
)]
fn metadata(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).metadata_anyhow().map(|_| ()))
}

#[test_case("/" => Ok(()); "ok root")]
#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    "err missing"
)]
fn symlink_metadata(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).symlink_metadata_anyhow().map(|_| ()))
}

#[test_case("/.." => Ok("/".to_string()))]
#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    "err missing"
)]
fn canonicalize(input: &str) -> Result<String, String> {
    stringify_error(
        Path::new(input)
            .canonicalize_anyhow()
            .and_then(|p| p.to_str_anyhow().map(String::from)),
    )
}

#[ignore]
#[test]
fn read_link_ok() -> anyhow::Result<()> {
    todo!(); // We need to create a symbolic link then test the target method.
}

#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    "err missing"
)]
fn read_link(input: &str) -> Result<String, String> {
    stringify_error(
        Path::new(input)
            .read_link_anyhow()
            .and_then(|p| p.to_str_anyhow().map(String::from)),
    )
}

#[test_case("/" => Ok(()); "ok")]
#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    );
    "err missing"
)]
fn read_dir(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).read_dir_anyhow().map(|_| ()))
}

#[test_case(
    "/this/path/should/not/exist",
    Path::new,
    "/this/path/also/should/not/exist",
    |p| format!(
        "while processing path {:?}: with copy_to \"/this/path/also/should/not/exist\": No such file or directory (os error 2)",
        p.display(),
    );
    "err non-existing to non-existing"
)]
#[test_case(
    tempfile::NamedTempFile::new().unwrap(),
    |nft| nft.path(),
    "/this/path/also/should/not/exist",
    |p| format!(
        "while processing path {:?}: with copy_to \"/this/path/also/should/not/exist\": No such file or directory (os error 2)",
        p.display(),
    );
    "err existing to non-existing"
)]
fn copy<T, IP, FMT>(input: T, into_path: IP, to: &str, fmt: FMT)
where
    IP: FnOnce(&T) -> &Path,
    FMT: FnOnce(&Path) -> String,
{
    let from = into_path(&input);
    let expected = fmt(from);

    let errstr = stringify_error(Path::new(from).copy_anyhow(to).map(|_| ()))
        .err()
        .unwrap();
    assert_eq!(expected, errstr,);
}

#[test_case(
    "/this/path/also/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/also/should/not/exist": No such file or directory (os error 2)"#,
    )
    ; "err within non-existing dir"
)]
fn create_dir(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).create_dir_anyhow())
}

#[test_case((); "permission denied")]
fn create_dir_all((): ()) -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    dir.path().set_readonly_anyhow(true)?;

    let path = dir.path().join("foo").join("bar");
    assert_error_desc_eq(
        path.create_dir_all_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: Permission denied (os error 13)",
            path.display(),
        ),
    );
    Ok(())
}

#[test_case((); "permission denied")]
fn hard_link((): ()) -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let path = dir.path().join("original");
    std::fs::write(&path, b"hello world")?;
    dir.path().set_readonly_anyhow(true)?;
    let link = dir.path().join("link");
    assert_error_desc_eq(
        path.hard_link_anyhow(&link),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: with link_to {:?}: Permission denied (os error 13)",
            path.display(),
            link.display(),
        ),
    );
    Ok(())
}

#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    )
    ; "missing"
)]
fn read(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).read_anyhow().map(|_| ()))
}

#[test_case(
    "/this/path/should/not/exist",
    Path::new,
    |pdisp| format!(
        "while processing path {pdisp:?}: No such file or directory (os error 2)",
    )
    ; "err missing"
)]
#[test_case(
    {
        use std::io::Write;

        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(b"not utf8: \xf3").unwrap();
        f.flush().unwrap();
        f
    },
    |f| f.path(),
    |pdisp| format!(
        "while processing path {pdisp:?}: stream did not contain valid UTF-8",
    )
    ; "err invalid utf8"
)]
fn read_to_string<T, TP, FMT>(input: T, path_of: TP, fmt: FMT)
where
    TP: Fn(&T) -> &Path,
    FMT: FnOnce(std::path::Display<'_>) -> String,
{
    let path = path_of(&input);
    let expected = fmt(path.display());
    let errdesc = stringify_error(path.read_to_string_anyhow().map(|_| ()))
        .err()
        .unwrap();
    assert_eq!(expected, errdesc);
}

#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    )
    ; "err non-existent"
)]
fn remove_dir(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).remove_dir_anyhow())
}

#[test_case((); "permission error")]
fn remove_dir_all((): ()) -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let a = dir.path().join("a");
    let b = a.join("b");
    let c = b.join("c");
    c.create_dir_all_anyhow()?;
    b.set_readonly_anyhow(true)?;

    assert_error_desc_eq(
        a.remove_dir_all_anyhow(),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: Permission denied (os error 13)",
            a.display(),
        ),
    );
    Ok(())
}

#[test_case(
    "/this/path/should/not/exist"
    => err_str(
        r#"while processing path "/this/path/should/not/exist": No such file or directory (os error 2)"#,
    )
    ; "non-existent"
)]
fn remove_file(input: &str) -> Result<(), String> {
    stringify_error(Path::new(input).remove_file_anyhow())
}

#[test_case((); "permission denied")]
fn rename((): ()) -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    a.create_dir_anyhow()?;
    dir.path().set_readonly_anyhow(true)?;

    assert_error_desc_eq(
        a.rename_anyhow(&b),
        // BUG: This error message is platform specific:
        &format!(
            "while processing path {:?}: with rename_to {:?}: Permission denied (os error 13)",
            a.display(),
            b.display(),
        ),
    );
    Ok(())
}

#[test_case((); "permission denied")]
fn write((): ()) -> anyhow::Result<()> {
    let dir = tempfile::TempDir::new()?;
    dir.path().set_readonly_anyhow(true)?;
    let path = dir.path().join("file");

    assert_error_desc_eq(
        path.write_anyhow("Hello World!"),
        // BUG: This error message is platform specific:
        &format!(
            "while writing to {:?}: Permission denied (os error 13)",
            path.display(),
        ),
    );
    Ok(())
}

fn assert_error_desc_eq<T>(res: anyhow::Result<T>, expected: &str) {
    let error = format!("{:#}", res.err().unwrap());
    assert_eq!(error, expected.trim_end());
}

fn err_str<T>(s: &str) -> Result<T, String> {
    Err(s.to_string())
}

fn stringify_error<T>(res: anyhow::Result<T>) -> Result<T, String> {
    res.map_err(|e| format!("{:#}", e))
}
