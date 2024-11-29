use std::fmt::{Formatter, Write};

pub(crate) trait ExecutableCommand {
    fn execute(&self) -> crate::error::CliResult<()>;
}
