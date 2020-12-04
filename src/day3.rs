use crate::utils::*;
use std::ops::Index;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Tile {
    Open,
    Tree,
}

struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    fn len(&self) -> usize {
        self.tiles.len()
    }
}

impl Index<usize> for Row {
    type Output = Tile;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.tiles[idx]
    }
}

impl FromStr for Row {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Row, Self::Err> {
        let tiles: Vec<Tile> = line
            .chars()
            .map(|c| match c {
                '.' => Tile::Open,
                '#' => Tile::Tree,
                _ => panic!("unknown tile"),
            })
            .collect();

        Ok(Row { tiles: tiles })
    }
}

pub fn solve(filepath: &'static str) -> u32 {
    let forest: Vec<Row> = read_lines::<Row>(filepath).collect();

    let mut hits = 0;
    let mut x = 0;
    let offset = 3;

    for row in forest {
        if row[x] == Tile::Tree {
            hits = hits + 1;
        }
        x = (x + offset) % row.len()
    }
    hits
}
