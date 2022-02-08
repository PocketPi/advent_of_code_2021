use std::{fs};

fn main() {
    let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
    // Step 1: get iterator from splitting on character.
    let values: Vec<&str> = contents.split('\n').collect();

    const WIDTH: usize = 12;

    let mut gamma: isize = 0;
    let mut epsilon: isize = 0;

    let mut result: [isize; WIDTH] = [0; WIDTH];

    for v in values {
        if let Ok(val) =  usize::from_str_radix(v, 2) {
            for (i, res) in result.iter_mut().enumerate().take(WIDTH) {
                if (val & 1<<i) > 0 {
                    *res += 1;
                } else {
                    *res -= 1;
                }
            }
        }
    }

    for (i, e) in result.iter().enumerate() {
        if e > &0 {
            gamma += 1<<i;
        } else {
            epsilon += 1<<i;
        }
    }
    println!("gamma: {} - epsilon: {} - Result: {}", gamma, epsilon, gamma * epsilon);
}