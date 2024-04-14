mod ky;
mod qwe;

use std::collections::HashMap;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::Read;
use std::{fmt, fs};

struct Lol {
    qwe: u32,
    asd: u32,
}

impl Lol {
    fn new(qwe: u32, asd: u32) -> Lol {
        Lol { qwe, asd }
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn name(arg: u32) -> i32 {
    123
}

fn main() {
    let greeting_file = File::open("hello.txt");
    println!("{}", 243324)
}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let asd = 3234;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;

    Ok(username)
}

fn qwe_as() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
}

fn qwe() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
