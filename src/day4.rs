use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub fn winners() {
    let input: String = fs::read_to_string("data/day4.txt").unwrap();
    let (first_line, other_lines) = input.split_once("\n").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let draws: Vec<i32> = re
        .captures_iter(first_line)
        .map(|x| x[0].parse().unwrap())
        .collect();

    let boards: Vec<_> = re
        .captures_iter(other_lines)
        .map(|x| x[0].parse().unwrap())
        .collect::<Vec<i32>>()
        .chunks(25)
        .map(|nums| {
            let mut res = Vec::new();
            for x in 0..5 {
                let mut row = HashSet::new();
                let mut col = HashSet::new();
                for y in 0..5 {
                    row.insert(nums[x * 5 + y]);
                    col.insert(nums[x + y * 5]);
                }
                res.push(row);
                res.push(col);
            }
            res
        })
        .collect();

    let mut results = Vec::new();
    for b in &boards {
        let mut drawn = HashSet::new();
        for x in 0..4 {
            // Add 4
            drawn.insert(draws[x]);
        }
        for d in 4..draws.len() {
            drawn.insert(draws[d]);
            if b.iter()
                .any(|row_or_column| row_or_column.intersection(&drawn).count() == 5)
            {
                let all_numbers =
                    b.iter()
                        .fold(HashSet::new(), |mut nums: HashSet<i32>, row_or_column| {
                            nums.extend(row_or_column);
                            nums
                        });
                let sum: i32 = all_numbers.difference(&drawn).into_iter().sum();
                let score: i32 = draws[d] * sum;
                results.push((d, score));
                break;
            }
        }
    }

    results.sort_unstable();

    println!("{:?}", results[0]);
    println!("{:?}", results.last().unwrap());
}
