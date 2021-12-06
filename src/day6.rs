use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn count_fish(days: usize) -> usize {
    let input = fs::read_to_string("data/day6.txt").unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let fish: Vec<usize> = re
        .captures_iter(&input)
        .map(|d| d[0].parse().unwrap())
        .collect();

    let freqs = fish.iter().fold(HashMap::new(), |mut map, f| {
        map.entry(f).and_modify(|count| *count += 1).or_insert(1);
        map
    });

    let mut counts = vec![0; 9];
    for (idx, item) in counts.iter_mut().enumerate() {
        *item = match freqs.get(&idx) {
            Some(&f) => f,
            None => 0,
        };
    }

    for _ in 0..days {
        let births = counts[0];
        counts.rotate_left(1);
        counts[6] += births;
    }

    counts.iter().sum()
}

pub fn part1() {
    println!("{}", count_fish(80));
}

pub fn part2() {
    println!("{}", count_fish(256));
}
