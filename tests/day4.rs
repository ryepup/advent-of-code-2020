use advent2020::day4;

#[test]
fn solve() {
    assert_eq!(2, day4::solve("./data/day4.test.txt"));
    assert_eq!(190, day4::solve("./data/day4.txt"));
}

#[test]
fn solve2() {
    assert_eq!(0, day4::solve2("./data/day4.test.txt"));
    assert_eq!(0, day4::solve2("./data/day4.txt"));
}
