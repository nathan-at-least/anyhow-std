use crate::process::{Child, ExitStatus, Output};
use anyhow::Context;
use std::process::Command;

/// Extend [std::process::Command] with [anyhow] methods
pub trait CommandAnyhow {
    /// Wrap [Command::spawn](std::process::Command::spawn), providing the command as error context
    fn spawn_anyhow(&mut self) -> anyhow::Result<Child>;

    /// Wrap [Command::output](std::process::Command::output), providing the command as error context
    fn output_anyhow(&mut self) -> anyhow::Result<Output>;

    /// Wrap [Command::status](std::process::Command::status), providing the command as error context
    fn status_anyhow(&mut self) -> anyhow::Result<ExitStatus>;

    /// Describe the command for error contexts
    fn anyhow_context(&self) -> String;
}

impl CommandAnyhow for Command {
    fn spawn_anyhow(&mut self) -> anyhow::Result<Child> {
        self.spawn()
            .map(|c| Child::from((c, self.anyhow_context())))
            .context(self.anyhow_context())
    }

    fn output_anyhow(&mut self) -> anyhow::Result<Output> {
        self.output()
            .map(|o| Output::wrap(o, self.anyhow_context()))
            .context(self.anyhow_context())
    }

    fn status_anyhow(&mut self) -> anyhow::Result<ExitStatus> {
        self.status()
            .map(|c| ExitStatus::from((c, self.anyhow_context())))
            .context(self.anyhow_context())
    }

    fn anyhow_context(&self) -> String {
        format!("command: {:?}", self)
    }
}
