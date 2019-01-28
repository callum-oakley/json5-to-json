use json5;
use serde_json::Value;
use std::error::Error;
use std::io::{self, Read};
use std::process;

fn run() -> Result<String, Box<Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    Ok(json5::from_str::<Value>(&buffer)?.to_string())
}

fn main() {
    match run() {
        Ok(s) => println!("{}", s),
        Err(err) => {
            eprintln!("json5-to-json: error: {}", err);
            process::exit(1);
        }
    }
}
