use std::collections::HashMap;
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

        let mut draws = data.split(";");
        if draws.all(| draw | analyze_draw(MAX_R, MAX_G, MAX_B, draw)) {
            sum + id
        } else {
            sum
        }
    });

    println!("{}", validgames);
}

// Analyze the contents of a draw to see if they meet the constraints.
fn analyze_draw(max_r: u32, max_g: u32, max_b: u32, draw: &str) -> bool {
    let mut color_maxes = HashMap::new();
    color_maxes.insert("red", max_r);
    color_maxes.insert("green", max_g);
    color_maxes.insert("blue", max_b);

    draw.split(",").all(| item | {
        // Ignore leading space.
        let mut data_tup = item.split(" ").skip(1);
        let (num, color) = (data_tup.next().unwrap(), data_tup.next().unwrap());
        let num: u32 = num.parse().unwrap();
        num <= color_maxes.get(color).unwrap().clone()
    })
}

// Pseudocode:
// For each line
// -- Tokenize on colon to get game ID
// -- Tokenize on semicolon to get individual instance
// -- Validate each instance.
