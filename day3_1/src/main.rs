use std::{fs};


fn main() {
    let contents = fs::read_to_string("test_input.txt")
    .expect("Something went wrong reading the file");
    // Step 1: get iterator from splitting on character.
    let values: Vec<&str> = contents.split('\n').collect();

    let mut gamma = 0;
    let mut epsilon = 0;

    let mut pos_0 = 0;
    let mut pos_1 = 0;
    let mut pos_2 = 0;
    let mut pos_3 = 0;
    let mut pos_4 = 0;

    for v in values {
        if let Ok(val) =  usize::from_str_radix(v, 2) {
            print!("{:05b} ", val);
            if ((val & 1<<0)>>0) == 1 {
                pos_0 += 1;
            } else {
                pos_0 -= 1;
            }
            if ((val & 1<<1)>>1) == 1 {
                pos_1 += 1;
            } else {
                pos_1 -= 1;
            }
            if ((val & 1<<2)>>2) == 1 {
                pos_2 += 1;
            } else {
                pos_2 -= 1;
            }
            if ((val & 1<<3)>>3) == 1 {
                pos_3 += 1;
            } else {
                pos_3 -= 1;
            }
            if ((val & 1<<4)>>4) == 1 {
                pos_4 += 1;
            } else {
                pos_4 -= 1;
            }

        }
    }

    println!("{} {} {} {} {}", pos_4>0, pos_3>0, pos_2>0, pos_1>0, pos_0>0);

    println!("gamma: {} - epsilon: {} - Result: {}", gamma, epsilon, gamma * gamma);
}