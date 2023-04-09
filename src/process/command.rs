use crate::process::{Child, ExitStatus, Output};
use anyhow::Context;
use std::process::Command;

pub trait CommandAnyhow {
    fn spawn_anyhow(&mut self) -> anyhow::Result<Child>;
    fn output_anyhow(&mut self) -> anyhow::Result<Output>;
    fn status_anyhow(&mut self) -> anyhow::Result<ExitStatus>;
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
