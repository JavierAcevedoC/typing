use colored::*;
use std::io::{stdin};

pub fn get_diff(input: &str, current_word: &str) {
    for (c1, c2) in input.chars().zip(current_word.chars()) {
        let mut chars_wrong = String::from("");
        let mut chars_ok = String::from("");
        if c1.ne(&c2) {
            chars_wrong.push(c1);
        } else {
            chars_ok.push(c1);
        }
        print!("{}",chars_ok.green());
        print!("{}",chars_wrong.red());
    }
    println!();
}

pub fn read_word_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).ok();
    input
}