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


// interacting with hashmaps for this problem was the biggest rust headache i've had so far
// i would say this is the first time i've had to rethink an algorithm because
// of the constraints of the borrow checker.
// so this is just all over the place. in addition to misreading the problem at first,
// i reconstructed it twice with some hacky string parsing.
fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut totals = 0u32;
    // if i was more comfortable with rust i think i would opt to use a trie here
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
                            wd.push_str("/");
                            cd = dirs.get_mut(&wd).unwrap().clone();
                        } else {
                            wd.push_str(dir);
                            wd.push_str("/");
                            if dirs.contains_key(&wd) {
                                cd = dirs.get_mut(&wd).unwrap().clone();
                            } else {
                                cd = Dir::new();
                            }
                            dirs.insert(wd.clone(), cd);
                        }
                    }
                }
                "dir" => {
                    let kid = split.next().unwrap();
                    let mut parent = wd.clone();
                    parent.push_str(kid);
                    dirs.get_mut(&wd).unwrap().children.push(parent.to_owned());
                }
                _ => {
                    dirs.get_mut(&wd).unwrap().size += u32::from_str_radix(head, 10).unwrap();
                }
            }
        }

        for (k,v) in dirs.iter() {
            println!("{}: {}, {:?}", k, v.size, v.children);
        }

        fn sum_children() -> u32 {
            let mut kid_weight = 0u32;
            for child in Self.children {
                kid_weight += child.sum_children();
            }
            Self.size += kid_weight;
            return Self.size;
        }
    }