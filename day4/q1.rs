use std::io::{BufReader, prelude::*};
use std::fs::File;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    // i have discovered while let
    while let Some(Ok(line)) = reader.next() {
        // TIL difference between single and double quotes
        // and that split takes Pattern which can be a char slice
        let mut elves = line.split([',', '-']);

        // there's probably a better way to do this
        let min1 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let max1 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let min2 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();
        let max2 = u8::from_str_radix(elves.next().unwrap(), 10).unwrap();

        println!("{min1}, {max1}; {min2}, {max2}");
        if (min1 >= min2 && max1 <= max2) || 
           (min2 >= min1 && max2 <= max1) {
            res += 1;
            println!("^^^ contained ^^^");
        }
    }
    println!("{res}");
}