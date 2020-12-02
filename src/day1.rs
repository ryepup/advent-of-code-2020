use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn solve(input: &'static str) -> Result<u32, io::Error> {
    let path = Path::new(input);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut entries = BTreeSet::new();
    for line in reader.lines() {
        if let Ok(entry) = line {
            println!("read '{}'", entry);
            let n = u32::from_str_radix(&entry, 10).expect("line was not a number");
            let target = 2020 - n;
            if entries.contains(&target) {
                return Ok(n * target);
            }
            entries.insert(n);
        }
    }
    panic!("no solution found");
}
