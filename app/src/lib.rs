use std::{error::Error, fs::File};

pub use config::Config;

mod config;

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    File::create("tests/data/rename/bar_file")?;
    Ok(())
}
