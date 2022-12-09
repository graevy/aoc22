use std::io::{BufReader, prelude::*, Result};
use std::fs::File;


// guess i'm doing generator -> String -> generator here
fn main() -> Result<()> {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    // the pallet contains all the boxes in a 2d array
    let mut pallet: Vec<Vec<char>> = Vec::new();

    // parse the stacks
    while let Some(Ok(line)) = reader.next() {
        if line == "".to_string() {
            break;
        }
        println!("{line}");
        let mut column = 0;
        let mut iter = line.chars();
        // what this really needs is slice.chunks() which i learned about
        // after coding this
        // array_chunks is also in the nightly rn
        while let Some(chr) = iter.next() {
            if column >= pallet.len() {
                pallet.push(Vec::<char>::new());
            }
            if chr == '[' {
                pallet[column].push(iter.next().unwrap());
                iter.nth(1);
            } else {
                iter.nth(2);
            }
            column += 1;
        }
    }
    // for column in pallet {
    //     println!("{:?}", column)
    // }

    while let Some(Ok(line)) = reader.next() {
        let mut instructions = line.chars();
        // finally getting lazy instantiation
        let quantity: u32;
        let q1 = instructions.nth(5).unwrap().to_digit(10).unwrap();
        let q2 = instructions.nth(0).unwrap();
        if q2 != ' ' {
            quantity = q1 * 10 + (q2.to_digit(10).unwrap());
            instructions.next();
        } else {
            quantity = q1;
        }
        println!("{q1}, {q2}, {quantity}");
    }
    Ok(())
}