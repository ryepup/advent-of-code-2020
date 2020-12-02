use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;
use std::path::Path;

fn read_numbers(filepath: &'static str) -> impl Iterator<Item = u32> {
    let path = Path::new(filepath);
    let file = File::open(&path).expect("could not read path");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.parse::<u32>().ok())
}

// O(N) algorithm:
// * keep a set of numbers we've seen before
// * read each line, convert to integer
// * figure out what the matching entry would be via subtraction
// * if we have that in the set, then multiply and quit
// * otherwise add to the set and move to next line

pub fn solve(input: &'static str) -> u32 {
    let mut entries = BTreeSet::new();
    for n in read_numbers(input) {
        let target = 2020 - n;
        if entries.contains(&target) {
            return n * target;
        }
        entries.insert(n);
    }
    panic!("no solution found");
}

// O(N^2) algorithm
// * pull all entries into a set
// * iterate through every element `a`
//   * calculate the maximum `b` such that a + b + c = 2020
//   * iterate through elements < `b`
//     * calculate `c` such that a + b + c = 2020
//     * if `c` exists in our entries, then we've found the solution

pub fn solve2(input: &'static str) -> u32 {
    let entries = BTreeSet::from_iter(read_numbers(input));

    for a in entries.iter() {
        let a_upper = 2020 - a;
        for b in entries.range(0..a_upper) {
            let c = 2020 - a - b;
            if entries.contains(&c) {
                return a * b * c;
            }
        }
    }
    panic!("no solution found");
}
