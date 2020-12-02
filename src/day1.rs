use crate::utils::*;
use std::collections::BTreeSet;
use std::iter::FromIterator;

// O(N) algorithm:
// * keep a set of numbers we've seen before
// * read each line, convert to integer
// * figure out what the matching entry would be via subtraction
// * if we have that in the set, then multiply and quit
// * otherwise add to the set and move to next line

pub fn solve(input: &'static str) -> u32 {
    let mut entries = BTreeSet::new();
    for n in read_lines::<u32>(input) {
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
    let entries = BTreeSet::from_iter(read_lines::<u32>(input));

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
