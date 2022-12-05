use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f).lines();
    let mut sum: u32 = 0;
    let mut max: u32 = 0;

    for line in reader {
        
        if let Ok(_res) = line {
            if _res == "" {
                sum = 0;
            }
            else {
                sum += _res.parse::<u32>().unwrap();
                if sum > max {
                    max = sum;
                    println!("{max}");
                }
            }
            // println!("{_res}, {sum}");
        }
        else { break; }
    }
}