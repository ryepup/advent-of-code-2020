use core::str::FromStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn read_lines<T: FromStr>(filepath: &'static str) -> impl Iterator<Item = T> {
    let path = Path::new(filepath);
    let file = File::open(&path).expect("could not read path");
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| line.parse::<T>().ok())
}
