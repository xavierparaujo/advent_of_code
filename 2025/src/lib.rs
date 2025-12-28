use std::io;
use std::error::Error;
use std::fs::File;
use std::fs;


pub fn read_input(path: &str) -> Result<Vec<String>,Box<dyn Error>>{
    let mut result = Vec::new();
    for line in fs::read_to_string("src/bin/".to_string() + path+ "/input.txt").unwrap().lines(){
        result.push(line.to_string())
    }
    return Ok(result)
}

fn main() {
    println!("Hello, world!");
}