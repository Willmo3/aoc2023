use std::{env, fs};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    println!("{:?}", find_numbers(&contents));
}

// Find where all the numbers are.
fn find_numbers(s: &str) -> Vec<Number> {
    let mut i: usize = 0;
    let mut start_index: Option<usize> = None;
    let mut end_index: usize;
    let mut retval : Vec<Number> = Vec::new();

    // Inclusive start index
    // Exclusive end index.
    for char in s.chars() {
        match start_index {
            None => {
                if char.is_digit(10) {
                    start_index = Some(i);
                };
            },
            Some(start) => {
                if !char.is_digit(10) {
                    end_index = i;
                    retval.push(Number { start_index: start, end_index } );
                    start_index = None;
                };
            }
        }

        i += 1;
    }

    retval

}

// loop through each character
// when number encountered, mark start index
// proceed until end index
// add number struct

#[derive(Debug)]
struct Number {
    start_index: usize,
    end_index: usize,
}

// Given an index collection of data:
// 1. Place into 2D array
// 2. Parse all numbers, getting start and end index.
// 3. For each number, check if there's an adjacent symbol.
// 4. If so, add.
