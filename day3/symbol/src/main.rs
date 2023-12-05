use std::{env, fs};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let contents: Vec<&str> = contents.split("\n").collect();

    println!("{:?}", find_numbers(&contents));
}

// Find where all the numbers are.
fn find_numbers(s: &Vec<&str>) -> Vec<Coords> {
    let mut i: usize = 0;
    let mut start_index: Option<(usize, usize)> = None;
    let mut end_index: (usize, usize);
    let mut retval : Vec<Coords> = Vec::new();

    for row in s.iter() {
        let mut j: usize = 0;
        for char in row.chars() {
            match start_index {
                None => {
                    if char.is_digit(10) {
                        start_index = Some((i, j));
                    };
                },
                Some(start) => {
                    if !char.is_digit(10) {
                        end_index = (i, j);
                        retval.push(Coords { start_index: start, end_index });
                        start_index = None;
                    };
                }
            }
            j += 1;
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
struct Coords {
    start_index: (usize, usize),
    end_index: (usize, usize)
}

// Given an index collection of data:
// 1. Place into 2D array
// 2. Parse all numbers, getting start and end index.
// 3. For each number, check if there's an adjacent symbol.
// 4. If so, add.
