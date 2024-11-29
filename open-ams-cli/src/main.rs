use clap::Parser;

pub(crate) mod cli;
pub(crate) mod error;
pub(crate) mod executors;

fn main() {
    let cli = cli::Cli::parse();
    cli.execute();
}
