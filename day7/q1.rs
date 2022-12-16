use std::io::{BufReader, prelude::*};
use std::fs::File;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut totals = 0u32;
    let mut wd = Vec::<&str>::new();

        while let Some(Ok(line)) = reader.next() {
            let mut split = line.split(' ');
            let mut size = 0u32;
            let head = split.next().unwrap();

            match head {
                "$" => {
                    if split.next().unwrap() == "cd" {
                        let dir = split.next().unwrap();
                        if dir == ".." {
                            wd.pop();
                        } else {
                            wd.push(dir);
                        }
                        totals += size;
                        size = 0;
                    }
                }
                "dir" => {
                    continue;
                }
                _ => {
                    size += u32::from_str_radix(head, 10).unwrap();
                }
            }
        }
    println!("{totals}");
    }
