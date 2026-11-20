use clap ::{Parser, Subcommand};

#[derive(Parser)]
pub struct CliArgs {
    #[command(subcommand)]
    pub(crate) command : Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    CurrentDir,
    Scan,
    Categorize
}

pub fn parse() -> CliArgs {
    CliArgs :: parse()
}