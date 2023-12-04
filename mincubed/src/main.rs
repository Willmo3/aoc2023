use std::collections::HashMap;
use std::fs;
use std::env;
fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let mut contents = fs::read_to_string(filename).unwrap();
    contents.pop(); // Remove trailing newline.
    let lines = contents.split("\n");

    let validgames = lines.fold(0, | sum, line | {
        let id_split = line.split(":");
        let data = id_split.skip(1).next().unwrap();
        let data = data.replace(";", ",");
        sum + power_game(&data)
    });

    println!("{}", validgames);
}

// TODO: SWITCH TO ANALYZE ENTIRE LINE

// Analyze the contents of a game to see if it meets the constainrs.
fn power_game(draw: &str) -> u32 {
    let mut color_maxes = HashMap::new();
    color_maxes.insert("red", 0);
    color_maxes.insert("green", 0);
    color_maxes.insert("blue", 0);

    draw.split(",").for_each(| item | {
        // Ignore leading space.
        let mut data_tup = item.split(" ").skip(1);
        let (num, color) = (data_tup.next().unwrap(), data_tup.next().unwrap());
        let num: u32 = num.parse().unwrap();
        println!("{}", num);

        if color_maxes.get(color).unwrap() < &num {
            color_maxes.insert(color, num);
        }
    });

    println!("{} {} {}", color_maxes.get("red").unwrap(), color_maxes.get("green").unwrap(), color_maxes.get("blue").unwrap());
    color_maxes.get("red").unwrap() * color_maxes.get("green").unwrap() * color_maxes.get("blue").unwrap()
}


// Pseudocode:
// For each line:
// -- Get the max cubes of each color.
// -- Multiply together
// -- Add to sum
