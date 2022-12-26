use std::io::{BufReader, prelude::*};
use std::fs::File;


// ATTEMPT 9
// the problem is called the interior mutability pattern. i spent a bit reading about it
// wrapping dirs in refcell permits interior mutability but i have to DFS one stack at a time
use std::collections::HashMap;
use std::cell::RefCell;

struct Dir {
    size: u32,
    children: Vec<String>,
} impl Dir {
    fn new() -> Self {
        Dir {
            size: 0u32,
            children: Vec::<String>::new()
        }
    }

    fn sum_children(&mut self, dir_map: &HashMap<String, RefCell<Self>>) -> u32 {
        let mut kid_size = 0u32;
        for child in self.children.iter() {
            kid_size += dir_map[child].borrow_mut().sum_children(dir_map);
        }
        self.size += kid_size;
        return self.size;
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let mut dirs = HashMap::<String, RefCell<Dir>>::new();
    dirs.insert("/".to_string(), RefCell::new(Dir::new()));

    let mut cd = "/".to_string();
    reader.next(); // skip adding root

        while let Some(Ok(line)) = reader.next() {
            let mut split = line.split(' ');
            let first = split.next().unwrap();
            let second = split.next().unwrap();
            println!("{first}, {second}: {cd}");

            match first {
                "$" => {
                    if second == "cd" {
                        let name = split.next().unwrap();
                        if name != ".." {
                            cd.push_str(name);
                            cd.push_str("/");
                            println!("cd augmented to {cd}");
                            if !dirs.contains_key(&cd) {
                                dirs.insert(cd.clone(), RefCell::new(Dir::new()));
                            }
                        } else {
                            cd = cd.rsplitn(3, "/").nth(2).unwrap().to_string();
                            cd.push_str("/");
                            println!("cd reduced to {cd}");
                        }
                    }
                }
                "dir" => {
                    dirs[&cd].borrow_mut().children.push(cd.clone());
                }
                _ => {
                    dirs[&cd].borrow_mut().size += u32::from_str_radix(first, 10).unwrap();
                }
            }
        }
        let mut head = dirs.get("/").unwrap().borrow_mut();
        println!("{:?}", head.size);
        head.sum_children(&dirs);
        println!("{:?}", head.size);
    }

// ATTEMPT 8
// so this is interesting. i can just revisit #7 with a HashMap of String:Dirs
// however. after the map is built, i then have to mutate the Dirs recursively to get their size linearly
// i have discovered that rust DOES NOT LIKE me mutating contents of hashmaps
// i must first get a mutable reference to the hashmap, and then another mutable reference to access its element
// since i can't have two simultaneous mutable references i don't have a solution here
// i feel like i'm misunderstanding the problem, because this sounds like something i should be able to do

// use std::collections::HashMap;

// struct Dir {
//     size: u32,
//     children: Vec<String>,
// } impl Dir {
//     fn new() -> Self {
//         Dir {
//             size: 0u32,
//             children: Vec::<String>::new()
//         }
//     }

//     fn sum_children(&mut self, dir_map: &mut HashMap<String, Self>) -> u32 {
//         let mut kid_size = 0u32;
//         for child in self.children.iter() {
//             kid_size += dir_map.get_mut(child).unwrap().sum_children(dir_map);
//         }
//         self.size += kid_size;
//         return self.size;
//     }
// }

// fn main() {
//     let f = File::open("input").unwrap();
//     let mut reader = BufReader::new(f).lines();

//     let mut dirs = HashMap::<String, Dir>::new();
//     dirs.insert("/".to_string(), Dir::new());

//     let mut cd = "/".to_string();

//         while let Some(Ok(line)) = reader.next() {
//             let mut split = line.split(' ');
//             let first = split.next().unwrap();
//             let second = split.next().unwrap();

//             match first {
//                 "$" => {
//                     if second == "cd" {
//                         let name = split.next().unwrap();
//                         if name != ".." {
//                             cd.push_str(name);
//                             cd.push_str("/");
//                             if !dirs.contains_key(&cd) {
//                                 dirs.insert(cd.clone(), Dir::new());
//                             }
//                         } else {
//                             cd = cd.rsplitn(3, "/").nth(2).unwrap().to_string();
//                         }
//                     }
//                 }
//                 "dir" => {
//                     dirs.get_mut(&cd).unwrap().children.push(cd.clone());
//                 }
//                 _ => {
//                     dirs.get_mut(&cd).unwrap().size += u32::from_str_radix(second, 10).unwrap();
//                 }
//             }
//         }
//     }


// ATTEMPT 7: Vec owns all elements. Dir structs contain indexes to the vec rather than references
// seems simple enough. dodges reference issues but feels like a cop-out; avoids using rust features
// maybe i could rewrite this using RefCell?
// abandoned because vecs would be O(n^2). you forgot the searching, you dork

// use std::collections::HashSet;

// struct Dir {
//     name: String,
//     size: u32,
//     parent: Option<u32>,
//     children: HashSet<u32>,
// } impl Dir {
//     fn new(name: String, parent: u32) -> Self {
//         Dir {
//             name: name,
//             size: 0u32,
//             parent: Some(parent),
//             children: HashSet::<u32>::new()
//         }
//     }
// }

// fn main() {
//     let f = File::open("input").unwrap();
//     let mut reader = BufReader::new(f).lines();

//     let mut dirs = Vec::<Dir>::new();
//     let mut dirs_idx = 0u32;
//     let mut root = Dir {
//         name: "/".to_string(),
//         size: 0u32,
//         parent: None,
//         children: HashSet::<u32>::new()
//     };
//     dirs.push(root);
//     dirs_idx += 1;

//     let mut cd = 0u32;

//         while let Some(Ok(line)) = reader.next() {
//             let mut split = line.split(' ');
//             let first = split.next().unwrap();
//             let second = split.next().unwrap();

//             match first {
//                 "$" => {
//                     if second == "cd" {
//                         let name = split.next().unwrap();
//                         if name != ".." {
//                             // add new dir to vec. 
//                             dirs.push(Dir::new(name.to_string(), cd));
//                             // its index is dirs_idx.
//                             dirs_idx += 1;
//                             // add that to current_dir's children.
//                             dirs[&cd].children.insert(dirs_idx.copy());
//                             // set that to current_dir.
//                             cd = dirs_idx.copy();
//                         } else {
//                             cd = dirs[&cd].parent.unwrap();
//                         }
//                     }
//                 }
//                 "dir" => {

//                 }
//                 _ => {}
//             }
//         }
// }


// ATTEMPT 6: structs own their own children and point to their parents
// failed because of reference mutability constraints

// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};


// #[derive(Eq,PartialEq)]
// struct Dir<'a> {
//     name: String,
//     size: u32,
//     parent: Option<&'a Dir<'a>>,
//     children: HashSet<Dir<'a>>,
// }

// impl Hash for Dir<'_> {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//         self.parent.hash(state);
//     }
// }

// impl<'b> Dir<'b> {
//     fn new(name: String, parent: &'b Dir) -> Self {
//         Dir {
//             name: name,
//             size: 0u32,
//             parent: Some(parent),
//             children: HashSet::<Dir>::new()
//         }
//     }

//     fn sum_children(&mut self) -> u32 {
//         let mut kid_size = 0u32;
//         for child in self.children.iter() {
//             kid_size += child.sum_children();
//         }
//         self.size += kid_size;
//         return self.size;
//     }
// }

// fn main() {
//     let f = File::open("input").unwrap();
//     let mut reader = BufReader::new(f).lines();

//     let mut head = Dir {
//         name: "/".to_string(),
//         size: 0u32,
//         parent: None,
//         children: HashSet::<Dir>::new()
//     };

//     let mut cd = &mut head;

//         while let Some(Ok(line)) = reader.next() {
//             let mut split = line.split(' ');
//             let first = split.next().unwrap();
//             let second = split.next().unwrap();

//             match first {
//                 "$" => {
//                     if second == "cd" {
//                         let name = split.next().unwrap();
//                         if name == ".." {
//                             cd = cd.parent.unwrap();
//                         } else {
//                             let next_dir = Dir::new(name.to_string(), cd);
//                             cd.children.insert(next_dir);
//                             cd = &mut next_dir;
//                         }
//                     }
//                 }

//                // didn't bother progressing further with this strategy
//                 _ => {}
//             }
//         }
// }


// ATTEMPTS 1-5: HashMap owns all items, is searched to return mutable items.
// what i would consider a standard approach to solving this problem in most languages.
// failed because of reference mutability constraints, circular lifetimes, and code re-use

// fn main() {
//     let f = File::open("input").unwrap();
//     let mut reader = BufReader::new(f).lines();

//     // let mut totals = 0u32;
//     // if i was more comfortable with rust i think i would opt to use a trie here
//     let mut dirs = HashMap::<String, Dir>::new();

//     // working directory
//     let mut wd = String::from("");

//         while let Some(Ok(line)) = reader.next() {
//             let mut split = line.split(' ');
//             let head = split.next().unwrap();

//             match head {
//                 // case: either a cd or ls command
//                 "$" => {
//                     // ls can be ignored
//                     if split.next().unwrap() == "cd" {
//                         let dir = split.next().unwrap();
//                         if dir == ".." {
//                             // go up a dir, e.g. from /foo/bar/ to /foo/
//                             wd = wd.rsplitn(3, "/").nth(2).unwrap().to_string();
//                             wd.push_str("/");
//                             // cd = dirs.get_mut(&wd).unwrap().clone();
//                         } else {
//                             // go down a dir by appending dir + /
//                             wd.push_str(dir);
//                             wd.push_str("/");
//                             // initialize an empty dir if this dir hasn't been seen yet
//                             if !dirs.contains_key(&wd) {
//                                 dirs.insert(wd.clone(), Dir::new());
//                             }
//                         }
//                     }
//                 }
                
//                 // case: we have to add a child directory to a parent dir so we can size them later
//                 "dir" => {
//                     let kid: &str = split.next().unwrap();
//                     let mut family: String = wd.clone();
//                     family.push_str(kid);

//                     if !dirs.contains_key(&family) {
//                         dirs.insert(family, Dir::new());
//                     }
                    
//                     let parent_dir = dirs.get(&wd).unwrap();
//                     let child_dir = dirs.get(&family).unwrap();
//                     parent_dir.children.push(child_dir);
//                 }

//                 // case: there's a filesize indicated that needs to get added to parent_dir
//                 _ => {
//                     let size = u32::from_str_radix(head, 10).unwrap();
//                     dirs.get(&wd).unwrap().inc_size(size);
//                 }
//             }
//         }


//         // for (k, v) in dirs.iter() {
//         //     println!("{}: {}, {:?}", k, v.size, v.children);
//         //     dirs.sum_children(v);
//         //     println!("{}: {}, {:?}", k, v.size, v.children);
//         // }
//     }
