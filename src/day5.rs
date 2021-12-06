use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn overlaps(ignore_diagonals: bool) -> usize {
    let mut points = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();
    let coords: Vec<Vec<i32>> = fs::read_to_string("data/day5.txt")
        .unwrap()
        .lines()
        .map(|l| re.captures_iter(l).map(|d| d[0].parse().unwrap()).collect())
        .collect();

    coords.iter().for_each(|c| {
        if !ignore_diagonals || (c[0] == c[2] || c[1] == c[3]) {
            let dx = (c[2] - c[0]).signum();
            let dy = (c[3] - c[1]).signum();

            let mut x = c[0];
            let mut y = c[1];
            while (x, y) != (c[2] + dx, c[3] + dy) {
                let entry = points.entry((x, y)).or_insert(0);
                *entry += 1;

                x += dx;
                y += dy;
            }
        }
    });

    let overlaps = points.iter().filter(|&(_k, v)| *v > 1).count();
    println!("{:?}", overlaps);
    overlaps
}

pub fn part1() {
    overlaps(true);
}

pub fn part2() {
    overlaps(false);
}
