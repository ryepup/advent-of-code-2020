extern crate regex;

use crate::utils::read_lines;
use regex::Regex;
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
                None => {
                    if pairs.len() == 0 {
                        return None;
                    } else {
                        // must have been the last line
                        return Some(pairs.into());
                    }
                }
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

fn is_valid_height<'a>(hgt: &'a &String) -> bool {
    // hgt (Height) - a number followed by either cm or in:
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(?P<hgt>\d+)(?P<uom>cm|in)$").unwrap();
    }
    if let Some(caps) = RE.captures(&hgt) {
        let n = caps["hgt"].parse::<u8>().unwrap();
        match &caps["uom"] {
            // If in, the number must be at least 59 and at most 76.
            "in" => (59..=76).contains(&n),
            // If cm, the number must be at least 150 and at most 193.
            "cm" => (150..=193).contains(&n),
            _ => false,
        }
    } else {
        false
    }
}

fn is_really_valid(passport: &Passport) -> bool {
    lazy_static! {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        // a nine-digit number, including leading zeroes.
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let is_valid = passport.byr.filter(|x| (1920..=2002).contains(x)).is_some()
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    && passport.iyr.filter(|x| (2010..=2020).contains(x)).is_some()
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    && passport.eyr.filter(|x| (2020..=2030).contains(x)).is_some()
    && passport.hgt.as_ref().filter(is_valid_height).is_some()
    && passport.hcl.as_ref().filter(|x| HCL_RE.is_match(x)).is_some()
    && passport.ecl.as_ref().filter(|x| ECL_RE.is_match(x)).is_some()
    && passport.pid.as_ref().filter(|x| PID_RE.is_match(x)).is_some();

    println!("is_really_valid {} {:?}", is_valid, passport);

    is_valid
}

pub fn solve2(filepath: &'static str) -> usize {
    println!("solving {}", filepath);
    let reader = PassportReader {
        lines: Box::new(read_lines(filepath)),
    };
    reader.filter(is_really_valid).count()
}
