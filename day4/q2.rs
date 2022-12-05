use std::io::{BufReader, prelude::*};
use std::fs::File;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    while let Some(Ok(line)) = reader.next() {
        let mut elves = line.split([',', '-']);

        let min1 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let max1 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let min2 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let max2 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();

        if !(max1 < min2 || max2 < min1) {
            res += 1;
        }
    }
    println!("{res}");
}