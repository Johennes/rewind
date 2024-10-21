use std::{fs, path::Path};

use anyhow::{ensure, Context, Result};

#[derive(clap::Args)]
pub(crate) struct LintFilenamesArgs {}

impl LintFilenamesArgs {
    pub(crate) fn run(&self) -> Result<()> {
        let failures = find_failures(Path::new("src"))?;
        ensure!(
            failures.is_empty(),
            format!(
                "The following files don't match the naming pattern:\n{}",
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
            if path.is_dir() {
                failures.append(&mut find_failures(&path)?);
            } else {
                let extension = path.extension().map(|s| s.to_string_lossy().to_string());
                if extension == Some("pdf".to_string()) || extension == Some("jpg".to_string()) {
                    let stem = path
                        .file_stem()
                        .context("Path should be representable as string")?
                        .to_string_lossy();
                    let num_underscores = stem.chars().filter(|c| *c == '_').count();
                    let num_whitespace = stem.chars().filter(|c| c.is_whitespace()).count();
                    let parts: Vec<&str> = stem.split("_").collect();
                    if num_whitespace > 0
                        || num_underscores != 3
                        || parts[0].contains(',')
                        || (parts[2] != "service-manual" && parts[2] != "owners-manual")
                    {
                        failures.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    Ok(failures)
}
