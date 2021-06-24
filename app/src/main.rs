use std::{env, process};

const HELP: &str = "\
rn 0.1.0
Thomas Le Duc <dev@tleduc.de>

rn is an intuitive and short command to rename files and directories.

Project home page: https://github.com/thled/rn


USAGE:
    rn OLD_NAME NEW_NAME

OLD_NAME:
    - The file/directory to rename.
    - Can contain an absolute or relative path.

NEW_NAME:
    The new name of the file/directory.

EXAMPLES:
    ./rn foo_file bar_file
    ./rn path/to/foo_file bar_file
    ./rn foo_dir bar_dir
    ./rn path/to/foo_dir bar_dir
    ./rn /absolute/path/to/foo_file bar_file
    ./rn /absolute/path/to/foo_dir bar_dir
";

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = match args.get(1) {
        Some(arg) => arg,
        _ => "",
    };
    if first_arg == "--help" {
        println!("{}", HELP);
        return;
    }

    let config = rn::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rn::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
