use std::fs;
use std::env;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut contents = fs::read_to_string(filename).unwrap();
    contents.pop(); // Remove trailing newline.
    let validgames = contents.split("\n").fold(0, | sum, line | {
        let mut id_split = line.split(":");
        let (id, data) = (id_split.next().unwrap(), id_split.next().unwrap());
        
        let id = id.split(" ").skip(1).next().unwrap();
        println!("{}", id);
        sum
    });
}

// Pseudocode:
// For each line
// -- Tokenize on colon to get game ID
// -- Tokenize on semicolon to get individual instance
// -- Validate each instance.
