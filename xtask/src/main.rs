mod lint_filenames;
mod lint_license;

use anyhow::Result;
use clap::{Parser, Subcommand};
use lint_filenames::LintFilenamesArgs;
use lint_license::LintLicenseArgs;

#[derive(Parser)]
struct CliParser {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    LintFilenames(LintFilenamesArgs),
    LintLicense(LintLicenseArgs),
}

fn main() -> Result<()> {
    match CliParser::parse().command {
        CliCommand::LintFilenames(args) => args.run(),
        CliCommand::LintLicense(args) => args.run(),
    }
}
