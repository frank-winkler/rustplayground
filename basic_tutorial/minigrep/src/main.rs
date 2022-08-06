use std::env;
use std::error::Error;
use std::fs;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    

    run(config).unwrap_or_else(|err| {
        println!("Problem reading from file: {}", err);
        process::exit(1);
    })  
}
