use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::HashSet;


fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    for stream in reader {
        let line = stream.unwrap();
        let (first, second) = line.split_at(line.len() >> 1);

        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();
        for char in first.chars() {
            first_set.insert(char);
        }
        for char in second.chars() {
            second_set.insert(char);
        }

        let intersection = first_set.intersection(&second_set).nth(0).unwrap();
        let mut value = *intersection as u32;
        if intersection.is_ascii_uppercase() {
            value -= 38;
        }
        else {
            value -= 96;
        }
        println!("{first}, {second}, {intersection}, {value}");
        res += value;
    }
    println!("{res}");
}