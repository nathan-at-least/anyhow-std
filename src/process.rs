//! Wrappers for [std::process] which provide commands in error contexts

mod child;
mod command;
mod exitstatus;
mod output;

pub use self::child::Child;
pub use self::command::CommandAnyhow;
pub use self::exitstatus::ExitStatus;
pub use self::output::Output;

#[cfg(test)]
mod tests;
