pub struct Config {
    pub old_name: String,
    pub new_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            old_name: args[1].clone(),
            new_name: args[2].clone(),
        })
    }
}
