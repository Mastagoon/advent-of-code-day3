use std::{fs, io::BufRead};

fn main() {
    println!("Hello, world!");
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let length = line.len();
        let (first_compartment, second_compartment) = line.split_at(length / 2);
        first_compartment.chars().into_iter().find(|c| {
            if second_compartment.contains(c) {
                return c;
            }
        });
        // let middle = length/2;
        // let splitter = line[5];
        // let [] = line.
    }
}
