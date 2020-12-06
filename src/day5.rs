use crate::utils::read_lines;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::ops::Range;

pub fn upper(r: Range<u32>) -> Range<u32> {
    Range {
        start: r.end - ((r.end - r.start) / 2),
        end: r.end,
    }
}

pub fn lower(r: Range<u32>) -> Range<u32> {
    Range {
        start: r.start,
        end: r.start + ((r.end - r.start) / 2),
    }
}

pub fn seat_id(spec: &str) -> u32 {
    let mut row: Range<u32> = 0..127;
    let mut seat: Range<u32> = 0..7;
    for c in spec.chars() {
        match c {
            'F' => {
                // lower half
                row = lower(row);
            }
            'B' => {
                // upper half
                row = upper(row);
            }
            'R' => {
                // upper half
                seat = upper(seat);
            }
            'L' => {
                // lower half
                seat = lower(seat);
            }
            _ => panic!("unknown char {}", c),
        }
    }
    (row.start * 8) + seat.start
}

pub fn solve(filepath: &'static str) -> u32 {
    read_lines(filepath).map(|x| seat_id(&x)).max().unwrap()
}

pub fn solve2(filepath: &'static str) -> u32 {
    let seat_ids = read_lines(filepath).map(|x| seat_id(&x));
    let set = BTreeSet::from_iter(seat_ids);

    for row in 0..127 {
        for seat in 0..7 {
            let seat_id: u32 = (row * 8) + seat;
            if set.contains(&seat_id) {
                continue;
            }

            if set.contains(&(seat_id + 1)) && set.contains(&(seat_id - 1)) {
                return seat_id;
            }
        }
    }
    panic!("no seat found")
}
