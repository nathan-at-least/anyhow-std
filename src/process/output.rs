use crate::process::ExitStatus;

#[derive(Debug)]
pub struct Output {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Output {
    pub(crate) fn wrap(output: std::process::Output, cmddesc: String) -> Self {
        Output {
            status: ExitStatus::from((output.status, cmddesc)),
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}
