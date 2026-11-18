use crate::error::Result;

#[macro_export]
macro_rules! default_config_file {
    () => {
        "config.json"
    };
}

#[derive(Debug, Default, serde::Deserialize)]
pub struct Config {
    width: u32,
    height: u32,
    render_scale: f64,
}

impl Config {
    pub fn new(content: &str) -> Result<Self> {
        let config: Self = serde_json::from_str(content)?;
        Ok(config)
    }
}
