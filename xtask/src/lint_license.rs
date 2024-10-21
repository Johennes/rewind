use anyhow::{ensure, Result};
use std::{
    fs::{metadata, read_to_string},
    path::PathBuf,
    str::FromStr,
};

#[derive(clap::Args)]
pub(crate) struct LintLicenseArgs {}

impl LintLicenseArgs {
    pub(crate) fn run(&self) -> Result<()> {
        let license_path = PathBuf::from_str("LICENSE")?;
        ensure!(
            metadata(&license_path).is_ok(),
            format!("{} should exist", license_path.to_string_lossy())
        );

        let license_md_path = PathBuf::from_str("src")?.join("license.md");
        ensure!(
            metadata(&license_md_path).is_ok(),
            format!("{} should exist", license_md_path.to_string_lossy())
        );

        let license = read_to_string(&license_path)?;
        let license: Vec<&str> = license.lines().collect();
        let license_md = read_to_string(&license_md_path)?;
        let license_md: Vec<&str> = license_md.lines().skip(2).collect();
        ensure!(&license == &license_md, "Contents should match");

        Ok(())
    }
}
