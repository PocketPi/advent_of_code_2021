use std::{fs, usize};

fn main() {
    let contents = fs::read_to_string("input")
    .expect("Something went wrong reading the file");
    // Step 1: get iterator from splitting on character.
    let values: Vec<&str> = contents.split_whitespace().collect();
    let mut count = 0;

    let mut refrence = match values[0].parse::<usize>() {
        Ok(v) => v,
        Err(v) => panic!("Not a valid value found as first element in input {:?}", v),
    };

    println!("Refrence: {}", refrence);

    for v in values {
        if let Ok(value) =v.parse::<usize>() {
            if value > refrence {
                count += 1;
                refrence = value;
                println!("{} -> Increse", value);
            } else {
                println!("{} -> Do noting", value);
                refrence = value;
            }
        }
    }
    println!("Result: {}", &count);
}