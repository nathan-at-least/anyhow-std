use crate::env;
use crate::testutils::{err_str, stringify_error};
use std::ffi::OsString;
use test_case::test_case;

#[test_case(
    "!% SHOULD NOT EXIST %!"
    => err_str(r#"environment variable "!% SHOULD NOT EXIST %!": environment variable not found"#)
)]
fn var(key: &str) -> Result<String, String> {
    stringify_error(env::var(key))
}

#[test_case(
    "!% SHOULD NOT EXIST %!"
    => err_str(r#"environment variable "!% SHOULD NOT EXIST %!": environment variable not found"#)
)]
#[test_case(
    "BAD = SIGN"
    => err_str(r#"environment variable "BAD = SIGN": environment variable contains '='"#)
)]
#[test_case(
    "BAD \0 CHAR"
    => err_str(r#"environment variable "BAD \0 CHAR": environment variable contains '\0'"#)
)]
fn var_os(key: &str) -> Result<OsString, String> {
    stringify_error(env::var_os(key))
}
