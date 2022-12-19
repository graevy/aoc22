use std::io::{BufReader, prelude::*};
use std::fs::File;

use std::collections::HashMap;


#[derive(Clone)]
struct Dir {
    size: u32,
    children: Vec<String>,
}

impl Dir {
    fn new() -> Self {
        Dir { size: 0u32, children: Vec::<String>::new() }
    }
}


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut size = 0u32;
    let mut totals = 0u32;
    let mut dirs = HashMap::<String, Dir>::new();

    // current directory and working directory
    let mut cd = Dir::new();
    let mut wd = String::from("");

        while let Some(Ok(line)) = reader.next() {
            let mut split = line.split(' ');
            let head = split.next().unwrap();

            match head {
                "$" => {
                    if split.next().unwrap() == "cd" {
                        let dir = split.next().unwrap();
                        if dir == ".." {
                            wd = wd.rsplitn(3, "/").nth(2).unwrap().to_string();
                            cd = dirs.get_mut(&wd).unwrap();
                        } else {
                            wd.push_str(dir);
                            wd.push_str("/");
                            if dirs.contains_key(&wd) {
                                cd = dirs.get_mut(&wd).unwrap();
                            } else {
                                cd = Dir::new();
                            }
                            dirs.insert(wd.clone(), cd.to_owned());
                        }
                        // println!("{:?}", wd);
                        if size >= 100000 {
                            totals += size;
                        }
                        size = 0;
                    }
                }
                "dir" => {
                    let kid = split.next().unwrap();
                    let mut parent = wd.clone();
                    parent.push_str(kid);
                    cd.children.push(parent.to_owned());
                }
                _ => {
                    cd.size += u32::from_str_radix(head, 10).unwrap();
                    // println!("{size}");
                }
            }
        }
    println!("{totals}");
    }