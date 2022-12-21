use std::io::{BufReader, prelude::*};
use std::fs::File;

use std::collections::HashMap;


struct Dir<'a> {
    size: u32,
    children: Vec<&'a Dir<'a>>,
}

impl Dir<'_> {
    fn new() -> Self {
        Dir { size: 0u32, children: Vec::<&Dir>::new() }
    }

    fn push_child(&mut self, child: &Dir) {
        self.children.push(child);
    }

    fn inc_size(&mut self, int: u32) {
        self.size += int;
    }

    fn sum_children(&mut self) -> u32 {
        let mut kid_size = 0u32;
        for child in self.children.iter_mut() {
            kid_size += child.sum_children();
        }
        self.size += kid_size;
        return self.size;
    }
}

// interacting with hashmaps for this problem was the biggest rust headache i've had so far
// i would say this is the first time i've had to rethink an algorithm because
// of the constraints of the borrow checker
fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    // let mut totals = 0u32;
    // if i was more comfortable with rust i think i would opt to use a trie here
    let mut dirs = HashMap::<String, Dir>::new();

    // working directory
    let mut wd = String::from("");

        while let Some(Ok(line)) = reader.next() {
            let mut split = line.split(' ');
            let head = split.next().unwrap();

            match head {
                // case: either a cd or ls command
                "$" => {
                    // ls can be ignored
                    if split.next().unwrap() == "cd" {
                        let dir = split.next().unwrap();
                        if dir == ".." {
                            // go up a dir, e.g. from /foo/bar/ to /foo/
                            wd = wd.rsplitn(3, "/").nth(2).unwrap().to_string();
                            wd.push_str("/");
                            // cd = dirs.get_mut(&wd).unwrap().clone();
                        } else {
                            // go down a dir by appending dir + /
                            wd.push_str(dir);
                            wd.push_str("/");
                            // initialize an empty dir if this dir hasn't been seen yet
                            if !dirs.contains_key(&wd) {
                                dirs.insert(wd.clone(), Dir::new());
                            }
                        }
                    }
                }
                // case: we have to add a child directory to a parent dir so we can size them later
                "dir" => {
                    let kid: &str = split.next().unwrap();
                    let mut family: String = wd.clone();
                    family.push_str(kid);

                    if !dirs.contains_key(&family) {
                        dirs.insert(family, Dir::new());
                    }
                    
                    dirs.get(&wd).unwrap().push_child(dirs.get(&family).unwrap());
                }
                // case: there's a filesize indicated that needs to get added to parent_dir
                _ => {
                    let size = u32::from_str_radix(head, 10).unwrap();
                    dirs.get(&wd).unwrap().inc_size(size);
                }
            }
        }


        // for (k, v) in dirs.iter() {
        //     println!("{}: {}, {:?}", k, v.size, v.children);
        //     dirs.sum_children(v);
        //     println!("{}: {}, {:?}", k, v.size, v.children);
        // }
    }
