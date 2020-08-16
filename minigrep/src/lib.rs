use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config<'a>, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
