use crate::OsStrAnyhow;
use std::ffi::OsStr;

#[test]
fn to_str_utf8() -> anyhow::Result<()> {
    let input = "hello world! 😀";
    let s = OsStr::new(input);
    let output = s.to_str_anyhow()?;
    assert_eq!(input, output);
    Ok(())
}

#[cfg(target_family = "unix")]
#[test]
fn to_str_invalid_utf8() -> anyhow::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let s = OsStr::from_bytes(b"invalid \xff utf8");
    let errdesc = format!("{:#}", s.to_str_anyhow().err().unwrap());
    assert_eq!(
        "while processing os string \"invalid \u{FFFD} utf8\": not valid utf8",
        errdesc
    );
    Ok(())
}

#[cfg(target_family = "unix")]
#[test]
fn to_str_invalid_utf8_big() -> anyhow::Result<()> {
    use std::os::unix::ffi::OsStrExt;

    let mut v = vec![];
    v.extend(b"invalid utf8 consisting of these ");
    for _ in 0..1024 {
        v.push(0xFF);
    }
    v.extend(b" non-codepoint bytes, which is also a very long string");

    let s = OsStr::from_bytes(v.as_slice());
    let errdesc = format!("{:#}", s.to_str_anyhow().err().unwrap());
    assert_eq!(
        "while processing os string \"invalid utf8 consisting of these \u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}\u{2772}\u{2026}\u{2773}tes, which is also a very long string\": not valid utf8",
        errdesc
    );
    Ok(())
}
