use std::{error::Error, fs::File};

pub use config::Config;

mod config;

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    File::create(format!("tests/data/rename/{}", _config.new_name))?;
    Ok(())
}
