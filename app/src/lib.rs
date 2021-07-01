use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    create_new_file(&config.source, &config.new_name)?;
    remove_old_file(&config.source)?;
    Ok(())
}

fn create_new_file(source: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
    let source = Path::new(source);
    let mut target = PathBuf::new();

    if has_path(source) {
        let path = source.parent().unwrap();
        target.push(path);
    }
    target.push(new_name);

    let content = fs::read(source)?;
    fs::write(&target, &content)?;
    Ok(())
}

fn has_path(source: &Path) -> bool {
    source.components().count() > 1
}

fn remove_old_file(source: &str) -> io::Result<()> {
    fs::remove_file(source)
}
