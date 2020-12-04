use core::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn parse_lines<T: FromStr>(filepath: &'static str) -> impl Iterator<Item = T> {
    read_lines(filepath).filter_map(|line| line.parse::<T>().ok())
}

pub fn read_lines(filepath: &'static str) -> impl Iterator<Item = String> {
    let path = Path::new(filepath);
    let file = File::open(&path).expect("could not read path");
    let reader = BufReader::new(file);

    reader.lines().filter_map(Result::ok)
}
