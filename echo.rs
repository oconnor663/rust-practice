extern crate docopt;

use std::env;

fn main() {
    let v: Vec<String> = env::args().skip(1).collect();
    println!("{}", v.connect(" "));
}
