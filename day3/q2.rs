use std::io::{BufReader, prelude::*};
use std::fs::File;

use std::collections::HashSet;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    loop {
        let stream = reader.next();
        if stream.is_none() {
            break;
        }
        let first = stream.unwrap().unwrap();

        let mut first_set = HashSet::new();
        for chr in first.chars() {
            first_set.insert(chr);
        }

        let second = reader.next().unwrap().unwrap();
        let mut second_set = HashSet::new();
        for chr in second.chars() {
            second_set.insert(chr);
        }

        let third = reader.next().unwrap().unwrap();
        let mut third_set = HashSet::new();
        for chr in third.chars() {
            third_set.insert(chr);
        }

        let mut first_intersection = HashSet::<char>::new();
        for chr in first_set.intersection(&second_set) {
            first_intersection.insert(*chr);
        }

        let badge = *first_intersection.intersection(&third_set).nth(0).unwrap();
        let mut value = badge as u32;
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