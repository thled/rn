use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

pub use config::Config;

mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let source = Path::new(&config.source);
    if source.is_file() {
        create_new_file(&source, &config.new_name)?;
        remove_old_file(&source)?;
    } else {
        create_new_dir(&config.new_name)?;
        remove_old_dir(&source)?;
    }
    Ok(())
}

fn remove_old_dir(source: &Path) -> Result<(), Box<dyn Error>> {
    fs::remove_dir(&source)?;
    Ok(())
}

fn create_new_dir(new_name: &str) -> Result<(), Box<dyn Error>> {
    fs::create_dir(new_name)?;
    Ok(())
}

fn create_new_file(source: &Path, new_name: &str) -> Result<(), Box<dyn Error>> {
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

fn remove_old_file(source: &Path) -> io::Result<()> {
    fs::remove_file(source)
}
