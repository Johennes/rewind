use std::{fs, path::Path};

use anyhow::{ensure, Result};
use image::ImageReader;

#[derive(clap::Args)]
pub(crate) struct LintImagesArgs {}

impl LintImagesArgs {
    pub(crate) fn run(&self) -> Result<()> {
        let failures = find_failures(Path::new("src"))?;
        ensure!(
            failures.is_empty(),
            format!(
                "The following images are incorrectly sized:\n{}",
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
            } else if extension == Some("jpg".to_string()) {
                let dimenstions = ImageReader::open(&path)?.into_dimensions()?;
                if dimenstions.1 != 350 {
                    failures.push(path.to_string_lossy().to_string());
                }
            }
        }
    }

    Ok(failures)
}
