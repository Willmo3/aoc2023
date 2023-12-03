use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let contents = fs::read_to_string(filename).unwrap();

    // Need to translate words to digits in order; i.e. first occurence of any words on a line gets replaced.
    // This in order requirement means we can't just call replace(), (see eightwo).
    let chars = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut charmap: HashMap<String, String> = HashMap::new();
    for i in 1..=9 {
        charmap.insert(i.to_string(), i.to_string());
        charmap.insert(chars[i].to_string(), i.to_string());
    }

    let calsum: u32 = contents.split("\n").fold(0, | sum, line | {
        let firstdigit = head_digit(line, &charmap);
        let lastdigit = tail_digit(line, &charmap);
        let numstr = format!("{}{}", firstdigit, lastdigit);
        sum + numstr.parse::<u32>().unwrap()
    });

    println!("{}", calsum);
}
    
// Return first digit in word
fn head_digit(line: &str, charmap: &HashMap<String, String>) -> String { 
    // I establishes the end bound
    for i in 1..=line.len() {
        // J establishes the start bound
        for j in 0..i {
            let str = line.get(j..i).unwrap();
            if let Some(digit) = charmap.get(str){
                return digit.to_string()
            }
        }
    }
    "0".to_string()
}

// Return last digit in word
fn tail_digit(line: &str, charmap: &HashMap<String, String>) -> String {
    // Iterate through every char in str in reverse.
    for i in (0..line.len()).rev() {
        for j in i..=line.len() {
            let str = line.get(i..j).unwrap();
            if let Some(digit) = charmap.get(str){
                return digit.to_string()
            }
        }
    }
    "0".to_string()
}