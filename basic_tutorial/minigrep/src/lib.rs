use std::error::Error;
use std::env;
use std::fs;

struct Config {
    query: String, 
    filename: String
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)
    Ok(println!("{:?}", contents) )
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };
        
        Ok(Config{query, filename})
   }
}

pub fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {line.contains(query)})
        .collect()
}

pub fn search_caser_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {

    contents
        .lines()
        .filter(|x| x.to_lowercase().contains(query))
        .collect()
}
