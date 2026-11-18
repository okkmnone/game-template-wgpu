use anyhow::{Context, Result};

use kit::config::Config;
use std::path::PathBuf;

fn main() -> Result<()> {
    let config = {
        let path = PathBuf::from("config.json");
        let content = std::fs::read_to_string(&path).context(concat!(
            "Failed to load file content from `",
            kit::default_config_file!(),
            "`",
        ))?;
        Config::new(&content)
            .with_context(|| format!("Failed to load configuration from {}", path.display()))?
    };
    kit::run(config)?;

    Ok(())
}
