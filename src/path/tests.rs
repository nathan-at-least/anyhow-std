use crate::PathAnyhow;
use indoc::indoc;
use std::path::Path;

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
