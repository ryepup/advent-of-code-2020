use advent2020::day3;

#[test]
fn solve() {
    assert_eq!(7, day3::solve("./data/day3.test.txt"));
    assert_eq!(268, day3::solve("./data/day3.txt"));
}

#[test]
fn solve2() {
    assert_eq!(336, day3::solve2("./data/day3.test.txt"));
    assert_eq!(3093068400, day3::solve2("./data/day3.txt"));
}
