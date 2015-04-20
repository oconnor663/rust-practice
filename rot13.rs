use std::io;
use std::io::prelude::*;

fn rot(c: char, start: char, end: char, rotation: u8) -> char {
    let len = end as u8 - start as u8;
    ((((c as u8 - start as u8) + rotation) % len) + start as u8) as char
}

fn main() {
    const ROTATION: u8 = 13;
    let mut input = String::new();
    let mut output = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for c in input.chars() {
        output.push(match c {
            'a'...'z' => rot(c, 'a', 'z', ROTATION),
            'A'...'Z' => rot(c, 'A', 'Z', ROTATION),
            _ => c,
        })
    }
    print!("{}", output);
}
