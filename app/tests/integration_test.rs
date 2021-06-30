use serial_test::serial;
use std::env;
use std::io::Write;
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

use rn::{self, Config};

#[test]
#[serial]
fn create_new_file() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    File::create(&source)?;
    let new_name = "bar_file";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", base_dir(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn different_file_name() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    File::create(&source)?;
    let new_name = "baz_file";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", base_dir(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn different_path() -> Result<(), Box<dyn Error>> {
    setup()?;
    let different_dir = format!("{}/{}", base_dir(), "different");
    fs::create_dir_all(&different_dir)?;
    let source = format!("{}/{}", different_dir, "foo_file");
    File::create(&source)?;
    let new_name = "bar_file";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/different/{}", base_dir(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn same_content() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    let mut file = File::create(&source).unwrap();
    file.write_all("content of file".as_bytes())?;
    let new_name = "bar_file";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", base_dir(), new_name);
    let content = fs::read_to_string(&target).unwrap();
    assert_eq!(content, "content of file");
    Ok(())
}

#[test]
#[serial]
fn remove_old_file() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    File::create(&source)?;
    let new_name = "bar_file";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    assert!(!Path::new(&source).exists());
    Ok(())
}

#[test]
#[serial]
fn no_path() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = format!("{}/{}", base_dir(), "foo_file");
    File::create(&source)?;
    let new_name = "bar_file";

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

fn change_dir_to_tests_data() -> Result<(), io::Error> {
    let cwd = env::current_dir().unwrap();
    if !cwd.ends_with("tests/data") {
        env::set_current_dir(format!("{}/tests/data", cwd.to_string_lossy()))?;
    }
    Ok(())
}

fn setup() -> Result<(), io::Error> {
    change_dir_to_tests_data()?;
    if Path::new(&base_dir()).exists() {
        fs::remove_dir_all(base_dir())?;
    }
    fs::create_dir_all(&base_dir())
}

fn base_dir() -> String {
    String::from("rename")
}
