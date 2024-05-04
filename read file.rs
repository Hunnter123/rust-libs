use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open(input())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input
}
