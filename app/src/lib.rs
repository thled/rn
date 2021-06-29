use std::{error::Error, fs::File, path::Path};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.old_name).parent().unwrap().to_string_lossy();
    File::create(format!("{}/{}", path,config.new_name))?;
    Ok(())
}
