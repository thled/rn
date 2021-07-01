use serial_test::serial;
use std::env;
use std::{
    error::Error,
    fs::{self, File},
    io,
    path::Path,
};

use rn::{self, Config};

#[test]
#[serial]
fn file_with_new_name() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_name";
    File::create(&source)?;
    let new_name = "new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    assert!(Path::new(&new_name).exists());
    Ok(())
}

#[test]
#[serial]
fn different_file_name() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_name";
    File::create(&source)?;
    let new_name = "diff_new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    assert!(Path::new(&new_name).exists());
    Ok(())
}

#[test]
#[serial]
fn same_content() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_name";
    let old_file_content = "content of file";
    fs::write(&source, &old_file_content)?;
    let new_name = "new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let new_file_content = fs::read_to_string(&new_name)?;
    assert_eq!(new_file_content, old_file_content);
    Ok(())
}

#[test]
#[serial]
fn remove_old_file() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_name";
    File::create(&source)?;
    let new_name = "new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    assert!(!Path::new(&source).exists());
    Ok(())
}

#[test]
#[serial]
fn relative_path() -> Result<(), Box<dyn Error>> {
    setup()?;
    let dir = "dir";
    fs::create_dir_all(&dir)?;
    let source = format!("{}/{}", dir, "old_name");
    File::create(&source)?;
    let new_name = "new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", dir, new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn absolute_path() -> Result<(), Box<dyn Error>> {
    setup()?;
    let cwd = env::current_dir()?;
    let source = format!("{}/{}", cwd.to_string_lossy(), "old_name");
    File::create(&source)?;
    let new_name = "new_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", cwd.to_string_lossy(), new_name);
    assert!(Path::new(&target).exists());
    Ok(())
}

#[test]
#[serial]
fn dir_with_new_name() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_dir_name";
    fs::create_dir(&source)?;
    let new_name = "new_dir_name";

    let config = create_config(&source, &new_name)?;
    rn::run(config)?;

    assert!(Path::new(&new_name).is_dir());
    Ok(())
}

#[test]
#[serial]
fn remove_old_dir() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_dir_name";
    fs::create_dir(source)?;
    let new_name = "new_dir_name";

    let config = create_config(source, new_name)?;
    rn::run(config)?;

    assert!(!Path::new(source).exists());
    Ok(())
}

#[test]
#[serial]
fn dir_with_path() -> Result<(), Box<dyn Error>> {
    setup()?;
    let dir = "dir";
    let source = format!("{}/{}", dir, "old_dir_name");
    fs::create_dir_all(&source)?;
    let new_name = "new_dir_name";

    let config = create_config(&source, new_name)?;
    rn::run(config)?;

    let target = format!("{}/{}", dir, new_name);
    assert!(Path::new(&target).is_dir());
    Ok(())
}

#[test]
#[serial]
fn dir_keeps_file() -> Result<(), Box<dyn Error>> {
    setup()?;
    let source = "old_dir_name";
    fs::create_dir(source)?;
    let content_file = "content_file";
    File::create(format!("{}/{}", source, content_file))?;
    let new_name = "new_dir_name";

    let config = create_config(source, new_name)?;
    rn::run(config)?;

    let new_dir_content = format!("{}/{}", new_name, content_file);
    assert!(Path::new(&new_dir_content).exists());
    Ok(())
}

fn create_config(source: &str, new_name: &str) -> Result<Config, &'static str> {
    let args = vec!["/bin/rn".to_owned(), source.to_owned(), new_name.to_owned()];
    rn::Config::new(&args)
}

fn change_dir_to_tests_data() -> Result<(), io::Error> {
    let cwd = env::current_dir()?;
    if !cwd.ends_with("tests/data") {
        env::set_current_dir(format!("{}/tests/data", cwd.to_string_lossy()))?;
    }
    Ok(())
}

fn setup() -> Result<(), io::Error> {
    change_dir_to_tests_data()?;
    remove_dir_contents(".")?;
    Ok(())
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?.path();
        if entry.is_file() {
            fs::remove_file(entry)?;
        } else {
            fs::remove_dir_all(entry)?;
        }
    }
    Ok(())
}
