#[test]
fn wrap_read_dir_item_err() {
    use std::io::{Error, ErrorKind::Other};

    if let Some(Err(err)) = super::wrap_read_dir_item(
        std::path::Path::new("fake-path"),
        Some(Err(Error::new(Other, "fake ReadDir iteration error"))),
    ) {
        assert_eq!(
            r#"while reading directory "fake-path": fake ReadDir iteration error"#,
            format!("{err:#}"),
        );
    } else {
        unreachable!();
    }
}
