use crate::env;
use crate::testutils::{err_str, stringify_error};
use test_case::test_case;

#[test_case(
    "!% SHOULD NOT EXIST %!"
    => err_str(r#"environment variable "!% SHOULD NOT EXIST %!": environment variable not found"#)
)]
fn var(key: &str) -> Result<String, String> {
    stringify_error(env::var(key))
}
