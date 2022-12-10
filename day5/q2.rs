use std::io::{BufReader, prelude::*};
use std::fs::File;



// guess i'm doing generator -> String -> generator here
fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    // the pallet contains all the boxes in a 2d array
    let mut pallet: Vec<Vec<char>> = Vec::new();

    // build the pallet from the first couple of lines
    while let Some(Ok(line)) = reader.next() {
        if line == "".to_string() {
            break;
        }
        println!("{line}");
        let mut column = 0;
        let mut iter = line.chars();
        // what this really needs is slice.chunks() which i learned about after coding this
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

    println!("initial pallet:");
    for idx in 0..pallet.len() {
        pallet[idx].reverse();
        println!("{:?}", pallet[idx]);
    }

    // now process move instructions
    while let Some(Ok(line)) = reader.next() {
        let mut instructions = line.split(" ");
        let mut stack = Vec::<char>::new();
        let quantity = usize::from_str_radix(instructions.nth(1).unwrap(), 10).unwrap();
        let source = usize::from_str_radix(instructions.nth(1).unwrap(), 10).unwrap() - 1;
        let dest = usize::from_str_radix(instructions.nth(1).unwrap(), 10).unwrap() - 1;

        println!("{line}");
        for _ in 0..quantity {
            stack.push(pallet[source].pop().unwrap());
        }
        
        while stack.len() > 0 {
            pallet[dest].push(stack.pop().unwrap());
        }

        for column in &pallet {
            println!("{:?}", column);
        }
    }
    let mut res = String::new();
    for column in pallet.as_slice() {
        res.push(*column.last().unwrap());
    }
    println!("\n\n{res}");
}