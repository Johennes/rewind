mod lint_filenames;
mod lint_images;
mod lint_license;
mod lint_metadata;

use anyhow::Result;
use clap::{Parser, Subcommand};
use lint_filenames::LintFilenamesArgs;
use lint_images::LintImagesArgs;
use lint_license::LintLicenseArgs;
use lint_metadata::LintMetadataArgs;

#[derive(Parser)]
struct CliParser {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    LintFilenames(LintFilenamesArgs),
    LintImages(LintImagesArgs),
    LintLicense(LintLicenseArgs),
    LintMetadata(LintMetadataArgs),
}

fn main() -> Result<()> {
    match CliParser::parse().command {
        CliCommand::LintFilenames(args) => args.run(),
        CliCommand::LintImages(args) => args.run(),
        CliCommand::LintLicense(args) => args.run(),
        CliCommand::LintMetadata(args) => args.run(),
    }
}
