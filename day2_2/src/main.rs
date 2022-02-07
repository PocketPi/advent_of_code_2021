use std::{fs};

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    // Step 1: get iterator from splitting on character.
    let values: Vec<&str> = contents.split('\n').collect();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for v in values {
        if let Some((command, value))= v.split_once(' ') {
            println!("{:?} - {:?}", command, value);
            let value: usize = value.parse().unwrap();
            match command {
                "forward"   => {
                    horizontal += value;
                    depth += aim * value;
                },
                "up"        => aim -= value,
                "down"      => aim += value,
                _           => println!("unknown command: {}", command),

            }
        }
    }

    println!("Horizontal: {} - Depth: {} - Result: {}", horizontal, depth, horizontal * depth);
}