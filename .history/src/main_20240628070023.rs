use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-010"));
}
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
