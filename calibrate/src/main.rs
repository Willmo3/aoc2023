use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;



fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut contents = fs::read_to_string(filename).unwrap();

    // Need to translate words to digits in order; i.e. first occurence of any words on a line gets replaced.
    // This in order requirement means we can't just call replace(), (see eightwo).
    let chars = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut charmap: HashMap<&'static str, &'static str> = HashMap::new();
    for i in 1..=9 {
        charmap.insert(i.to_string(), i.to_string());
        charmap.insert(chars[i].to_string(), i.to_string());
    }
    // zero is special case since words are not supposed to be mapped.
    charmap.insert("0".to_string(),"0".to_string());

    let calsum: u32 = contents.split("\n").fold(0, | sum, line | {
        let firstdigit = head_digit(line, &charmap);
        let lastdigit = tail_digit(line, &charmap);
        
        let numstr = format!("{}{}", firstdigit, lastdigit);
        sum + numstr.parse::<u32>().unwrap()
    });

    println!("{}", calsum);
}

// RETURNING STRING AS TO RESULT IN CLONED VALUE.

// Return first digit in word
fn head_digit(line: &str, charmap: &HashMap<String, String>) -> String {
    for i in 1..line.len() {
        let str = line.get(..i).unwrap();
        if let Some(digit) = charmap.get(str){
            return digit.clone();
        }
    }
    "0".to_string()
}

// Return last digit in word
fn tail_digit(line: &str, charmap: &HashMap<String, String>) -> String {
    for i in (line.len()..0).rev() {
        let str = line.get(i..).unwrap();
        if let Some(digit) = charmap.get(&str.to_string()){
            return digit.clone();
        }
    }
    "0".to_string()
}