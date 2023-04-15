use crate::process::{ExitStatus, Output};
use anyhow::Context;
use std::ops::Deref;
use std::process::{ChildStderr, ChildStdin, ChildStdout};

/// Wrap [std::process::Child] to provide the command as error context
#[derive(Debug)]
pub struct Child {
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
    child: std::process::Child,
    cmddesc: String,
}

impl From<(std::process::Child, String)> for Child {
    fn from((mut child, cmddesc): (std::process::Child, String)) -> Self {
        Child {
            stdin: child.stdin.take(),
            stdout: child.stdout.take(),
            stderr: child.stderr.take(),
            child,
            cmddesc,
        }
    }
}

impl Deref for Child {
    type Target = std::process::Child;

    fn deref(&self) -> &Self::Target {
        &self.child
    }
}

impl Child {
    /// Override [std::process::Child::kill] with the command as error context
    pub fn kill(&mut self) -> anyhow::Result<()> {
        self.child.kill().context(self.cmddesc.clone())
    }

    /// Override [std::process::Child::wait] with the command as error context
    pub fn wait(&mut self) -> anyhow::Result<ExitStatus> {
        self.child
            .wait()
            .map(|es| ExitStatus::from((es, self.cmddesc.clone())))
            .context(self.cmddesc.clone())
    }

    /// Override [std::process::Child::try_wait] with the command as error context
    pub fn try_wait(&mut self) -> anyhow::Result<Option<ExitStatus>> {
        self.child
            .try_wait()
            .map(|optes| optes.map(|es| ExitStatus::from((es, self.cmddesc.clone()))))
            .context(self.cmddesc.clone())
    }

    /// Override [std::process::Child::wait_with_output] with the command as error context
    pub fn wait_with_output(self) -> anyhow::Result<Output> {
        self.child
            .wait_with_output()
            .map(|o| Output::wrap(o, self.cmddesc.clone()))
            .context(self.cmddesc)
    }
}
