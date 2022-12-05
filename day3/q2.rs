use std::io::{BufReader, prelude::*, Result};
use std::fs::File;

use std::collections::HashSet;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    loop {
        let stream = reader.next().unwrap();
        if stream.is_err() {
            break;
        }
        let first = stream.unwrap();

        let mut first_set = HashSet::new();
        for char in first.chars() {
            first_set.insert(char);
        }

        let second = reader.next().unwrap().unwrap();
        let mut second_set = HashSet::new();
        for char in second.chars() {
            second_set.insert(char);
        }

        let third = reader.next().unwrap().unwrap();
        let mut third_set = HashSet::new();
        for char in third.chars() {
            third_set.insert(char);
        }

        let mut first_intersection = HashSet::new();
        for char in first_set.intersection(&second_set) {
            first_intersection.insert(char);
        }

        let badge = *first_intersection.intersection(&third_set).nth(0).unwrap();
        let mut value = *badge as u32;
        if badge.is_ascii_uppercase() {
            value -= 38;
        }
        else {
            value -= 96;
        }
        println!("{first}, {second}, {third}, {badge}, {value}");
        res += value;
    }
    println!("{res}");
}