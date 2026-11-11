use anyhow::{Context, Result};

use core::Config;

use std::path::PathBuf;

fn main() -> Result<()> {
    let config = {
        let path = PathBuf::from("config.json");
        let content = std::fs::read_to_string(&path).context("Failed to load file content")?;
        let config = Config::new(&content)
            .with_context(|| format!("Failed to load configuration from {}", path.display()))?;
        config
    };
    core::run(config)?;

    Ok(())
}
