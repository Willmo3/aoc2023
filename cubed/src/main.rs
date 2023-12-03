use std::fs;
use std::env;

static MAX_R: u32 = 12;
static MAX_G: u32 = 13;
static MAX_B: u32 = 14;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut contents = fs::read_to_string(filename).unwrap();
    contents.pop(); // Remove trailing newline.
    let lines = contents.split("\n");

    let validgames = lines.fold(0, | sum, line | {
        let mut id_split = line.split(":");
        let (id, data) = (id_split.next().unwrap(), id_split.next().unwrap());
        
        let id: &str = id.split(" ").skip(1).next().unwrap();
        let id: u32 = id.parse().unwrap();

        let draws = data.split(";");
        for draw in draws {
            analyze_draw(MAX_R, MAX_G, MAX_B, draw)
        }
        sum
    });
}

// Analyze the contents of a draw to see if they meet hte containts.
fn analyze_draw(max_r: u32, max_g: u32, max_b: u32, draw: &str) {
    println!("{}", draw);
}

// Pseudocode:
// For each line
// -- Tokenize on colon to get game ID
// -- Tokenize on semicolon to get individual instance
// -- Validate each instance.
