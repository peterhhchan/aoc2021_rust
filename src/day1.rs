use std::fs;

pub fn part1() -> usize {
    let readings = fs::read_to_string("data/day1.txt").unwrap();

    let r: Vec<i32> = readings.lines().map(|s| s.parse().unwrap()).collect();
    let count = r.windows(2).filter(|w| w.get(0) < w.get(1)).count();

    println! {"{}", count};
    count
}

pub fn part2() -> usize {
    let readings = fs::read_to_string("data/day1.txt").unwrap();

    let r: Vec<i32> = readings.lines().map(|s| s.parse().unwrap()).collect();
    let count = r.windows(4).filter(|w| w.get(0) < w.get(3)).count();

    println! {"{}", count};
    count
}
