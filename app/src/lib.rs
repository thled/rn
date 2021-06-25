use std::error::Error;

pub use config::Config;

mod config;

pub fn run(_config: Config) -> Result<(), Box<dyn Error>> {
    println!("todo");
    Ok(())
}
