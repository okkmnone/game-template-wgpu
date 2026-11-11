pub mod error;

pub use glam;

use crate::error::AppResult;

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    fullscreen: bool,
}

impl Config {
    pub fn new(content: &str) -> AppResult<Self> {
        let config: Self = serde_json::from_str(content)?;
        Ok(config)
    }
}

pub fn run(_config: Config) -> AppResult<()> {
    println!("{}", std::any::type_name_of_val(&run));

    Ok(())
}
