use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    create_new_file(&config.source, &config.new_name)?;
    // remove_old_file(&config.source)?;

    Ok(())
}

fn create_new_file(source: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
    // let path = Path::new(source).parent().unwrap().to_string_lossy();
    // let mut target = File::create(format!("{}/{}", path, new_name)).unwrap();
    let mut target = File::create(new_name).unwrap();
    // let content = fs::read(source)?;
    // target.write_all(&content)?;
    Ok(())
}

fn remove_old_file(source: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_file(source)?;
    Ok(())
}
