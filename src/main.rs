use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
#[macro_use]
mod test_utils;
mod puzzle_01;
mod puzzle_02;
mod puzzle_03;
mod puzzle_04;

const PUZZLE_NUMBER: u8 = 4;
const SOLVERS: [fn(&str) -> String; 4] = [puzzle_01::f, puzzle_02::f, puzzle_03::f, puzzle_04::f];

fn main() {
    let mut f = File::open(format!("input/puzzle_{:02}.txt", PUZZLE_NUMBER)).unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    let output = SOLVERS[(PUZZLE_NUMBER - 1) as usize](&input);
    println!("{}", output);
}
