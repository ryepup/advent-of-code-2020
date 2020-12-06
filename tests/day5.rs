use advent2020::day5;

#[test]
fn seat_id() {
    assert_eq!(357, day5::seat_id("FBFBBFFRLR"));
    assert_eq!(567, day5::seat_id("BFFFBBFRRR"));
    assert_eq!(119, day5::seat_id("FFFBBBFRRR"));
    assert_eq!(820, day5::seat_id("BBFFBBFRLL"));
}

#[test]
fn lower() {
    assert_eq!(0..63, day5::lower(0..127));
    assert_eq!(32..47, day5::lower(32..63));
    assert_eq!(44..45, day5::lower(44..47));
    assert_eq!(44..44, day5::lower(44..45));
}

#[test]
fn upper() {
    assert_eq!(32..63, day5::upper(0..63));
    assert_eq!(40..47, day5::upper(32..47));
    assert_eq!(44..47, day5::upper(40..47));
}

#[test]
fn solve() {
    assert_eq!(947, day5::solve("./data/day5.txt"));
}

#[test]
fn solve2() {
    assert_eq!(636, day5::solve2("./data/day5.txt"));
}
