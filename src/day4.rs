use regex::Regex;
use std::collections::HashSet;
use std::fs;

// Implemented a different technique compared to my clojure solution.
//
// 1. Determined the number each board won with
// 2. Generated a tuple (index of the number, score) for each board
// 3. Sorted the tuples

pub fn winners() {
    let input: String = fs::read_to_string("data/day4.txt").unwrap();
    let (first_line, other_lines) = input.split_once("\n").unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let draws: Vec<i32> = re
        .captures_iter(first_line)
        .map(|x| x[0].parse().unwrap())
        .collect();

    // In retrospect the following is kinda silly, I don't need to
    // generate 10 sets.  Rather I can simply bruteforce check each of
    // the 10 possible winning combinations using a .all
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
        for &x in draws.iter().take(4) {
            // Cannot win with only 4 numbers
            drawn.insert(x);
        }
        for (idx, &n) in draws.iter().enumerate().skip(4) {
            drawn.insert(n);
            if b.iter()
                .any(|row_or_column| row_or_column.is_subset(&drawn))
            {
                let sum: i32 = b
                    .iter()
                    .fold(HashSet::new(), |mut nums: HashSet<i32>, row_or_column| {
                        nums.extend(row_or_column);
                        nums
                    })
                    .difference(&drawn)
                    .into_iter()
                    .sum();

                let score: i32 = n * sum;

                results.push((idx, score));
                break;
            }
        }
    }

    results.sort_unstable();

    println!("{:?}", results[0]);
    println!("{:?}", results.last().unwrap());
}
