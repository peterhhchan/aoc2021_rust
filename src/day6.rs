use regex::Regex;
use std::fs;

pub fn count_fish(days: usize) -> usize {
    let input = fs::read_to_string("data/day6.txt").unwrap();

    let fish: Vec<usize> = Regex::new(r"\d+")
        .unwrap()
        .captures_iter(&input)
        .map(|d| d[0].parse().unwrap())
        .collect();

    let mut counts = fish.iter().fold(vec![0; 9], |mut counts, &f| {
        counts[f] += 1;
        counts
    });

    for _ in 0..days {
        let births = counts[0];
        counts.rotate_left(1);
        counts[6] += births;
    }

    counts.iter().sum()
}

pub fn part1() {
    println!("{}", count_fish(80)); //387413
}

pub fn part2() {
    println!("{}", count_fish(256)); //1738377086345
}
