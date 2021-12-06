use std::fs;

pub fn readings(window: usize) {
    let readings = fs::read_to_string("data/day1.txt").unwrap();
    let r: Vec<i32> = readings.lines().map(|s| s.parse().unwrap()).collect();
    let count = r.windows(1 + window).filter(|&w| w[0] < w[window]).count();

    println! {"{}", count};
}

pub fn part1() {
    readings(1); // 1184
}

pub fn part2() {
    readings(3); // 1158
}
