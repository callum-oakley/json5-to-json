extern crate json5;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read};
use std::process;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn run() -> Result<String, Box<Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let value = json5::from_str::<Value>(&buffer)?;

    let s = json5::to_string(&value)?;

    Ok(s)
}

fn main() -> () {
    match run() {
        Ok(s) => println!("{}", s),
        Err(err) => {
            eprintln!("sub4: error: {}", err);
            process::exit(1);
        }
    }
}
