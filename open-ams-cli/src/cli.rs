use crate::{
    error::CliResult,
    executors::{version::VersionExecutor, ExecutableCommand},
};

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Version,
}

impl Cli {
    pub fn execute(&self) -> CliResult<()> {
        let executable = match self.command {
            Command::Version => VersionExecutor::new(),
        };

        executable.execute()
    }
}
