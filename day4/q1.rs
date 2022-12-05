use std::io::{BufReader, prelude::*};
use std::fs::File;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut res: u32 = 0;

    // i have discovered while let
    while let Some(Ok(line)) = reader.next() {
        let mut elves = line.split([',', '-']);
        // let mut bounds = Vec::<&str>::with_capacity(4);

        let start1 = elves.next().unwrap();
        let end1 = elves.next().unwrap();
        let start2 = elves.next().unwrap();
        let end2 = elves.next().unwrap();

        println!("{start1}, {end1}; {start2}, {end2}");
        if (start1 >= start2 && end1 <= end2) || 
           (start2 >= start1 && end2 <= end1) {
            res += 1;
            println!("^^^ contained ^^^");
        }
    }
    println!("{res}");
}