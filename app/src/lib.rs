use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.source)
        .parent()
        .unwrap()
        .to_string_lossy();

    let mut new_file = File::create(format!("{}/{}", path, config.new_name)).unwrap();
    let content = fs::read(&config.source)?;
    new_file.write_all(&content)?;

    fs::remove_file(&config.source)?;

    Ok(())
}
