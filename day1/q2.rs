use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f).lines();
    let mut sum: u32 = 0;
    let mut max: u32 = 0;
    let mut mid: u32 = 0;
    let mut min: u32 = 0;

    for line in reader {
        if let Ok(_res) = line {
            if _res == "" {
                sum = 0;
                continue;
            }
            else {
                sum += _res.parse::<u32>().unwrap();
                if sum <= min { 
                    continue; 
                } else if sum > max {
                    min = mid;
                    mid = max;
                    max = sum;
                } else if sum > mid {
                    min = mid;
                    mid = sum;
                } else {
                    min = sum;
                }
            }
            println!("{sum}, {min}, {mid}, {max}");

        }
        else {
            break;
        }
    }
    println!( "{}", (min + mid + max).to_string() );
}