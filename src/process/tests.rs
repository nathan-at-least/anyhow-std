use crate::process::CommandAnyhow;
use std::process::Command;
use test_case::test_case;

#[test_case(Command::spawn_anyhow)]
#[test_case(Command::output_anyhow)]
#[test_case(Command::status_anyhow)]
fn unknown_process<F, T>(cb: F)
where
    F: FnOnce(&mut Command) -> anyhow::Result<T>,
{
    let mut cmd = Command::new("/! we assume this program does not exist !/");
    cmd.arg("ARG");

    let r = cb(&mut cmd);
    assert_eq!(
        format!("{:#}", r.err().unwrap()),
        // BUG: Platform specific error message:
        r#"command: "/! we assume this program does not exist !/" "ARG": No such file or directory (os error 2)"#,
    );
}
