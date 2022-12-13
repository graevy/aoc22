use std::io::{BufReader, prelude::*};
use std::fs::File;

use std::collections::HashMap;


fn main() {
    let f = File::open("input").unwrap();
    let mut reader = BufReader::new(f).lines();

    let consecutive_uniques: usize = 4;

        // this one's actually only one line. didn't get that from the prompt. oops
        while let Some(Ok(line)) = reader.next() {
            // my first implementation, which i abandoned, was O(nk) time, O(n) space
            // my second implementation (eof) was O(n+k) time and space
            // my third implementation here is O(n+k) time and O(k) space
            // 2 and 3 are more or less the same but using iterators vs slice.windows
            let back = line.chars();
            let mut front = line.chars();
            let mut freq_map = HashMap::<char, u32>::with_capacity(consecutive_uniques + 1);
            let mut idx = consecutive_uniques;

            // build the initial frequency map. this is the O(k) space
            for _ in 0..consecutive_uniques {
                let chr = front.next().unwrap();
                *freq_map.entry(chr).or_insert(0) += 1;
            }
            // i guess we might just run into k unique elems immediately
            if freq_map.len() >= consecutive_uniques {
                println!("index {}: {:?}", idx, freq_map);
                break;
            }
            for (head, tail) in front.zip(back) {
                // insert head into frequency counter
                *freq_map.entry(head).or_insert(0) += 1;
                // remove tail from frequency counter
                if freq_map[&tail] == 1 {
                    freq_map.remove(&tail);
                } else {
                    *freq_map.get_mut(&tail).unwrap() -= 1;
                }
                // increment index
                idx += 1;
                // check to see if we have k unique elems in freq map
                if freq_map.len() >= consecutive_uniques {
                    println!("index {}: {:?}", idx, freq_map);
                    break;
                }
            }
        }
}


    //         let line_vec = line.chars().collect::<Vec<char>>();
    //         let wins = line_vec.windows(consecutive_uniques + 1);


    //         // let mut window_start: usize = 0;
    //         // let mut window_end: usize = consecutive_uniques;
    //         let mut map = HashMap::<&char, u32>::with_capacity(5);

    //         // bootstrap the first k elems
    //         for idx in 0..consecutive_uniques {
    //             let chr = &line_vec[idx];
    //             *map.entry(chr).or_insert(0) += 1;
    //         }
    //         if map.len() >= consecutive_uniques {
    //             return;
    //         }
    //         println!("start map: {:?}", map);

    //         for window in wins {
    //             // decrement window start; remove if 0
    //             let first = &window.first().unwrap();
    //             if map[first] == 1 {
    //                 map.remove(first);
    //             } else {
    //                 *map.get_mut(first).unwrap() -= 1;
    //             }

    //             // increment window end; add if not in map
    //             let last = &window.last().unwrap();
    //             *map.entry(last).or_insert(0) += 1;

    //             if map.len() >= consecutive_uniques {
    //                 println!("res!: {:?}", map);
    //                 break;
    //             }
    //             println!("{:?}", map)
    //         }
    //         // this doesn't actually check the last index but there's no need to
    //         if map.len() < consecutive_uniques {
    //             println!("no matches");
    //         }
    //     }
    // }
