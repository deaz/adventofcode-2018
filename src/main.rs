use std::fs::File;
use std::io::prelude::*;

mod puzzle_01;

const PUZZLE_NUMBER: u8 = 1;
const SOLVERS: [fn(&str) -> String; 1] = [puzzle_01::f];

fn main() {
    let mut f = File::open(format!("input/puzzle_{:02}.txt", PUZZLE_NUMBER)).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let output = SOLVERS[(PUZZLE_NUMBER - 1) as usize](&input);
    println!("{}", output);
}
