use std::ops::Deref;

#[derive(Debug, derive_more::From, derive_more::Into)]
pub struct ExitStatus {
    es: std::process::ExitStatus,
    cmddesc: String,
}

impl Deref for ExitStatus {
    type Target = std::process::ExitStatus;

    fn deref(&self) -> &Self::Target {
        &self.es
    }
}

impl ExitStatus {
    pub fn exit_ok_anyhow(&self) -> anyhow::Result<()> {
        if self.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("error exit status").context(format!(
                "status: {}",
                self.code()
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "n/a".to_string())
            )))
        }
    }

    pub fn exit_anyhow(&self) -> ! {
        let code = match self.exit_ok_anyhow() {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("{:#}", e);
                self.code().unwrap_or(-1)
            }
        };
        std::process::exit(code)
    }
}

impl std::fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.es.fmt(f)
    }
}
