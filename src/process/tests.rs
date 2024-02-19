use crate::process::{CommandAnyhow, ExitStatus};
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

#[test]
fn exit_ok_error() -> anyhow::Result<()> {
    use std::process::Stdio;

    let es: ExitStatus = Command::new("cargo")
        .arg("! we assume this arg does not exit !")
        .stderr(Stdio::null())
        .status_anyhow()?;

    assert_eq!(
        format!("{:#}", es.exit_ok().err().unwrap()),
        // BUG: Platform specific error message:
        r#"status: 101: error exit status"#,
    );

    Ok(())
}
