use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() {
    println!("Enter the filename you want to create:");

    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();

    let filename = filename.trim();

    let mut file = File::create(filename).unwrap();

    println!("Enter the content to write to the file ({}):", filename);

    let mut content = String::new();
    io::stdin().read_line(&mut content).unwrap();

    file.write(content.as_bytes()).unwrap();
}
