use serial_test::serial;
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

use rn::{self, Config};

#[test]
#[serial]
fn rename_file() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    let new_name = "bar_file";
    File::create(&source)?;

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", base_dir(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn rename_to_another_name() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    let new_name = "baz_file";
    File::create(&source)?;

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", base_dir(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

fn create_config(source: &str, new_name: &str) -> Result<Config, &'static str> {
    let args = vec!["/bin/rn".to_owned(), source.to_owned(), new_name.to_owned()];
    rn::Config::new(&args)
}

fn setup() -> Result<(), io::Error> {
    if Path::new(&base_dir()).exists() {
        fs::remove_dir_all(base_dir())?;
    }
    fs::create_dir_all(&base_dir())
}

fn base_dir() -> String {
    String::from("tests/data/rename")
}
