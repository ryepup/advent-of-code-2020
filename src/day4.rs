use crate::utils::read_lines;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,    // (Birth Year)
    iyr: Option<u16>,    // (Issue Year)
    eyr: Option<u16>,    // (Expiration Year)
    hgt: Option<String>, // (Height) TODO: look up units of measure
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<u64>,    // (Country ID)
}

impl From<HashMap<String, String>> for Passport {
    fn from(map: HashMap<String, String>) -> Self {
        Passport {
            byr: get::<u16>("byr", &map),
            iyr: get::<u16>("iyr", &map),
            eyr: get::<u16>("eyr", &map),
            hgt: map.get("hgt").map(|x| x.to_string()),
            hcl: map.get("hcl").map(|x| x.to_string()),
            ecl: map.get("ecl").map(|x| x.to_string()),
            pid: map.get("pid").map(|x| x.to_string()),
            cid: get::<u64>("cid", &map),
        }
    }
}

fn get<T: FromStr>(key: &'static str, map: &HashMap<String, String>) -> Option<T> {
    match map.get(key) {
        Some(val) => val.parse::<T>().ok(),
        _ => None,
    }
}

struct PassportReader {
    lines: Box<dyn Iterator<Item = String>>,
}

impl Iterator for PassportReader {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pairs = HashMap::new();
        loop {
            let line = match self.lines.next() {
                Some(s) => s,
                None => return None,
            };
            if line.len() == 0 {
                return Some(pairs.into());
            }
            for pair in line.split(' ') {
                let kv: Vec<&str> = pair.split(':').collect();
                pairs.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
    }
}

fn is_valid(passport: &Passport) -> bool {
    passport.byr.is_some()
        && passport.ecl.is_some()
        && passport.eyr.is_some()
        && passport.hcl.is_some()
        && passport.hgt.is_some()
        && passport.iyr.is_some()
        && passport.pid.is_some()
}

pub fn solve(filepath: &'static str) -> usize {
    let reader = PassportReader {
        lines: Box::new(read_lines(filepath)),
    };
    reader.filter(is_valid).count()
}

pub fn solve2(filepath: &'static str) -> u32 {
    todo!()
}
