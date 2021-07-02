use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source = Path::new(&config.source);
    rename(&source, &config.new_name)?;
    Ok(())
}

fn rename(source: &Path, new_name: &str) -> Result<(), Box<dyn Error>> {
    let target = build_target(source, new_name);
    fs::rename(&source, &target)?;
    Ok(())
}

fn build_target(source: &Path, new_name: &str) -> PathBuf {
    let mut target = PathBuf::new();
    if has_path(source) {
        let path = source.parent().unwrap();
        target.push(path);
    }
    target.push(new_name);
    target
}

fn has_path(source: &Path) -> bool {
    source.components().count() > 1
}
