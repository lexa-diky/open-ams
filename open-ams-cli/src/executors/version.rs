use colored::Colorize;

pub(crate) struct VersionExecutor {}

impl VersionExecutor {
    pub fn new() -> Self {
        VersionExecutor {}
    }
}

impl crate::executors::ExecutableCommand for VersionExecutor {
    fn execute(&self) -> crate::error::CliResult<()> {
        let table = [
            ("📄", "open-ams-cli", "0.0.1-preview"),
            ("🏛️ ", "open-ams-contract", "0.0.1-preview"),
            ("👷", "open-ams-c", "0.0.1-preview"),
        ];

        println!(
            "{}  {}: {}\n",
            "🚧",
            "your-project".bold(),
            "<unknown>".green()
        );
        for (icon, element, version) in table {
            println!("{}  {}: {}", icon, element.bold(), version.green())
        }

        Ok(())
    }
}
