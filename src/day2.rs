// Each line gives the password policy and then the password. The password
// policy indicates the lowest and highest number of times a given letter must
// appear for the password to be valid. For example, 1-3 a means that the
// password must contain a at least 1 time and at most 3 times.
extern crate regex;

use crate::utils::*;
use regex::Regex;
use std::ops::RangeInclusive;
use std::str::FromStr;

// algorithm: O(N)
// * parse each line of file into a struct
// * test each for policy compliance
// * count the wins

struct Entry {
    is_compliant: bool,
}

impl FromStr for Entry {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<min>\d)-(?P<max>\d) (?P<l>[a-zA-Z]): (?P<pass>.+)$").unwrap();
        }
        let caps = RE.captures(line).expect("failed to parse!");
        let policy = RangeInclusive::new(
            caps["min"].parse::<usize>().unwrap(),
            caps["max"].parse::<usize>().unwrap(),
        );
        let letter = &caps["l"];
        let password = &caps["pass"];
        let occurrences = password.matches(letter).count();

        let entry = Entry {
            is_compliant: policy.contains(&occurrences),
        };

        Ok(entry)
    }
}

pub fn solve(filepath: &'static str) -> usize {
    read_lines::<Entry>(filepath)
        .filter(|e| e.is_compliant)
        .count()
}
