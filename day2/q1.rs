use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use std::collections::HashMap;


fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f).lines();

    let mut points: u32 = 0;

    let charmap = HashMap::from([
        ("A", "rock"),
        ("X", "rock"),
        ("B", "paper"),
        ("Y", "paper"),
        ("C", "scissors"),
        ("Z", "scissors")
    ]);

    let pointmap = HashMap::from([
        ("rock",1),
        ("paper",2),
        ("scissors",3),
    ]);

    let resultmap = HashMap::from([
        (("rock","rock"), 3),
        (("rock","paper"), 6),
        (("rock","scissors"), 0),
        (("paper","rock"), 0),
        (("paper","paper"), 3),
        (("paper","scissors"), 6),
        (("scissors", "rock"), 6),
        (("scissors", "paper"), 0),
        (("scissors", "scissors"), 3)
    ]);

    for stream in reader {
        let line = stream.unwrap();
        let sides: Vec<&str> = line.split(" ").collect();
        let me = charmap.get(&sides[1]).unwrap();
        let elf = charmap.get(&sides[0]).unwrap();

        points += pointmap.get(me).unwrap();
        points += resultmap.get(&(elf, me)).unwrap();

        let s = points.to_string();
        println!("elf: {elf}, me: {me}, total: {s}");
    }
}