#![allow(unused)] // For beginnesr only - tush doesn't need this

// Create a crate error
mod error;
mod prelude;
mod utils;

fn main() -> Result<()>{
    println!("Hello Tush");

    Ok(())
}

// use regex::Regex;

// fn main() {
//     let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
//     println!("Did our date match? {}", re.is_match("2014-01-01"));
// }
/*
fn main() {
    println!("Hello, world!");
    println!(); // prints just a newline
    println!("hello there!");
    println!("format {} arguments", "some");
    let local_variable = "many";
    println!("format {local_variable} arguments");
}
*/
