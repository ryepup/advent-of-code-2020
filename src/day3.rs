use crate::utils::*;

#[derive(PartialEq, Eq)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => panic!("unknown tile"),
        }
    }
}

fn parse_row(line: String) -> Vec<Tile> {
    line.chars().map(Tile::from).collect()
}

fn parse_forest(filepath: &'static str) -> Vec<Vec<Tile>> {
    read_lines(filepath).map(parse_row).collect()
}

pub fn solve(filepath: &'static str) -> u32 {
    let forest = parse_forest(filepath);

    travel(&forest, 1, 3)
}

pub fn solve2(filepath: &'static str) -> u32 {
    let forest = parse_forest(filepath);

    travel(&forest, 1, 1)
        * travel(&forest, 1, 3)
        * travel(&forest, 1, 5)
        * travel(&forest, 1, 7)
        * travel(&forest, 2, 1)
}

fn travel(forest: &Vec<Vec<Tile>>, down: usize, right: usize) -> u32 {
    let mut hits = 0;
    let mut x = 0;
    let mut y = 0;

    while y < forest.len() {
        if forest[y][x] == Tile::Tree {
            hits = hits + 1;
        }
        x = (x + right) % forest[y].len();
        y = y + down;
    }

    hits
}
