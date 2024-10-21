mod lint_license;

use anyhow::Result;
use clap::{Parser, Subcommand};
use lint_license::LintLicenseArgs;

#[derive(Parser)]
struct CliParser {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    LintLicense(LintLicenseArgs),
}

fn main() -> Result<()> {
    match CliParser::parse().command {
        CliCommand::LintLicense(args) => args.run(),
    }
}
