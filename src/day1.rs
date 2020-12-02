use std::collections::BTreeSet;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

// O(N) algorithm:
// * keep a set of numbers we've seen before
// * read each line, convert to integer
// * figure out what the matching entry would be via subtraction
// * if we have that in the set, then multiply and quit
// * otherwise add to the set and move to next line

pub fn solve(input: &'static str) -> Result<u32, io::Error> {
    let path = Path::new(input);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut entries = BTreeSet::new();
    for line in reader.lines() {
        if let Ok(entry) = line {
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
