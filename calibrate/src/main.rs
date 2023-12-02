use std::env;
use std::fs;
use regex::Regex;



fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    // Match all single digit strings.
    // Since we know all strings will be single digit, no need to cast into char.
    let re = Regex::new(r"\d").unwrap();

    let calsum: u32 = contents.split("\n").fold(0, | sum, line | {
        let mut digits = re.find_iter(line);
        let firstdigit = match digits.next() {
            None => "0",
            Some(digit) => digit.as_str()
        };
        let lastdigit = match digits.last() {
            None => firstdigit,
            Some(digit) => digit.as_str()
        };
        
        let numstr = format!("{}{}", firstdigit, lastdigit);
        sum + numstr.parse::<u32>().unwrap()
    });

    println!("{}", calsum);
}
