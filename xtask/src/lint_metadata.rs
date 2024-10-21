use anyhow::{ensure, Result};
use std::{fs, path::Path, process::Command};

#[derive(clap::Args)]
pub(crate) struct LintMetadataArgs {}

impl LintMetadataArgs {
    pub(crate) fn run(&self) -> Result<()> {
        let failures = find_failures(Path::new("src"))?;
        ensure!(
            failures.is_empty(),
            format!(
                "The following files contain metadata that should be scrubbed with mat2:\n{}",
                failures.join("\n")
            )
        );
        Ok(())
    }
}

fn find_failures(dir: &Path) -> Result<Vec<String>> {
    let mut failures = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let extension = path.extension().map(|s| s.to_string_lossy().to_string());

            if path.is_dir() {
                failures.append(&mut find_failures(&path)?);
            } else if extension == Some("pdf".to_string()) || extension == Some("jpg".to_string()) {
                let output = Command::new("mat2")
                    .arg("--show")
                    .arg(&path)
                    .output()
                    .expect("mat2 should not fail to run");
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                if !output.status.success() || !stdout.trim().starts_with("No metadata found in ") {
                    failures.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(failures)
}
