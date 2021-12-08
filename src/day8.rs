use regex::Regex;
use std::collections::HashSet;
use std::fs;
pub fn part1() {
    let input: String = fs::read_to_string("data/day8.txt").unwrap();
    let re = Regex::new(r"\w+").unwrap();

    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            let digits: Vec<String> = re.captures_iter(l).map(|d| d[0].parse().unwrap()).collect();
            digits
        })
        .collect();

    let count: usize = lines
        .iter()
        .map(|digits| {
            digits[10..]
                .iter()
                .filter(|d| matches!(d.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum();

    println!("{:?}", count);
}

fn chars(s: &String) -> HashSet<&u8> {
    let mut set = HashSet::new();
    for c in s.as_bytes() {
        set.insert(c);
    }
    set
}

fn find_by_count(digits: &[String], count: usize) -> (&String, HashSet<&u8>) {
    let res = digits.iter().find(|d| d.len() == count).unwrap();
    (res, chars(res))
}

pub fn part2() {
    let input: String = fs::read_to_string("data/day8.txt").unwrap();
    let re = Regex::new(r"\w+").unwrap();

    let lines: Vec<Vec<String>> = input
        .lines()
        .map(|l| {
            let digits: Vec<String> = re.captures_iter(l).map(|d| d[0].parse().unwrap()).collect();
            digits
        })
        .collect();

    let result: i32 = lines
        .iter()
        .map(|digits| {
            let display: &[String] = &digits[..10];
            let (_, one) = find_by_count(display, 2);
            let (_, four) = find_by_count(display, 4);

            let output: Vec<i32> = digits[10..]
                .iter()
                .map(|d| {
                    let chars = chars(d);
                    let match_ones = chars.intersection(&one).count();
                    let match_fours = chars.intersection(&four).count();

                    match (d.len(), match_ones, match_fours) {
                        (6, 2, 3) => 0,
                        (2, _, _) => 1,
                        (5, 1, 2) => 2,
                        (5, 2, _) => 3,
                        (4, _, _) => 4,
                        (5, 1, 3) => 5,
                        (6, 1, _) => 6,
                        (3, _, _) => 7,
                        (7, _, _) => 8,
                        (6, 2, 4) => 9,
                        _ => panic!(),
                    }
                })
                .collect();
            output[0] * 1000 + output[1] * 100 + output[2] * 10 + output[3]
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();

    println!("{:?}", result);
}
