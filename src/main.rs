mod qwe;
mod ky;

use std::collections::HashMap;
use std::fmt;
use std::fmt::format;
use std::io;
use std::io::Split;
use std::mem;
use std::process::id;
use rand::Rng;

struct Lol {
    qwe: u32,
    asd: u32
}

impl Lol {
    fn new(qwe: u32, asd: u32) -> Lol {
        Lol {
            qwe,
            asd
        }
    }
}

fn main() {
    let lol = String::from("hello world");

    let res = qwe(lol);

    print!("{}", res);
}


fn qwe(q: String) -> String {
    let mut temp = q.split_whitespace();

    let mut result = String::new();

    for idx in temp {
        if let Some(first_char) = idx.chars().next() {
            if is_vowel(first_char) {
                result.push_str(&format!("{}-hay ", idx));
            } else {
                result.push_str(&format!("{}-{}ay ", &idx[1..], first_char));
            }
        }
    }
    return result.trim().to_string(); // обрезаем последний пробел и преобразу
}


fn is_vowel(letter: char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    vowels.contains(&letter)
}