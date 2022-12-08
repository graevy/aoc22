use std::io::{BufReader, prelude::*};
use std::fs::File;


// struct Pallet {
//     stacks: Vec<Vec<&str>>,
// }

// impl Pallet {
//     fn add_to_pallet(&self, chr) {

//     }
// }


// guess i'm doing generator -> String -> generator here
fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut pallet: Vec<Vec<char>> = Vec::new();

    // parse the stacks
    while let Some(Ok(line)) = reader.next() {
        if line == "\n".to_string() {
            break;
        }
        println!("{line}");
        let mut spaces = line.as_bytes().chunks(4);
        let mut space = 0;
        while let Some(chonk) = spaces.next() {
            if chonk[0] == '[' as u8 {
                if space > pallet.len() - 1 {
                    pallet.push(Vec::<char>::new());
                }
                pallet[space].push(chonk[space + 1] as char);
            }
            space += 1;
        }

    }
}