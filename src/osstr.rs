use anyhow::Context;
use std::borrow::Cow;
use std::ffi::OsStr;

const LONG_STRING_DISPLAY_LIMIT: usize = 80;

pub trait OsStrAnyhow {
    fn to_str_anyhow(&self) -> anyhow::Result<&str>;
}

impl<P> OsStrAnyhow for P
where
    P: AsRef<OsStr>,
{
    fn to_str_anyhow(&self) -> anyhow::Result<&str> {
        let sref = self.as_ref();
        sref.to_str()
            .ok_or_else(|| anyhow::Error::msg("not valid utf8"))
            .with_context(|| {
                format!(
                    "while processing os string {:?}",
                    truncate_long_strings(sref.to_string_lossy())
                )
            })
    }
}

fn truncate_long_strings<'a>(s: Cow<'a, str>) -> Cow<'a, str> {
    let sref = s.as_ref();
    let charcnt = sref.chars().count();

    if charcnt <= LONG_STRING_DISPLAY_LIMIT {
        s
    } else {
        const HALF: usize = LONG_STRING_DISPLAY_LIMIT / 2;

        dbg!(Cow::from(format!(
            "{}\u{2772}\u{2026}\u{2773}{}",
            sref.chars().take(HALF).collect::<String>(),
            sref.chars().skip(charcnt - HALF + 3).collect::<String>(),
        )))
    }
}

#[cfg(test)]
mod tests;
