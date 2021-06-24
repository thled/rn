use serial_test::serial;
use std::error::Error;

use rn;

#[test]
#[serial]
fn rename_file() -> Result<(), Box<dyn Error>> {
    // todo
    let executable_name = "/bin/rn";
    let old_name = "foo_file";
    let new_name = "bar_file";
    let args = vec![
        executable_name.to_owned(),
        old_name.to_owned(),
        new_name.to_owned(),
    ];
    let config = rn::Config::new(&args)?;

    rn::run(config)?;

    Ok(())
}
