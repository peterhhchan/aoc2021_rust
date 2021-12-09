use regex::Regex;
use std::collections::HashSet;
use std::fs;

fn neighbors((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    vec![(x + 1, y), (x, y + 1), (x, y - 1), (x - 1, y)]
        .into_iter()
        .filter(|(x, y)| (0..100).contains(x) && (0..100).contains(y))
        .collect()
}

fn low_points(board: &Vec<Vec<i32>>) -> (Vec<(i32, i32)>, i32) {
    let mut low_points = Vec::new();
    let mut risk_levels = 0;
    for (x, row) in board.iter().enumerate() {
        for (y, &height) in row.iter().enumerate() {
            if neighbors((x as i32, y as i32))
                .iter()
                .all(|&(a, b)| board[a as usize][b as usize] > height)
            {
                low_points.push((x as i32, y as i32));
                risk_levels += height + 1;
            }
        }
    }

    (low_points, risk_levels)
}

pub fn part1() {
    let input: String = fs::read_to_string("data/day9.txt").unwrap();
    let re = Regex::new(r"\d").unwrap();

    let board: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let line: Vec<i32> = re
                .captures_iter(line)
                .map(|d| d[0].parse().unwrap())
                .collect();
            line
        })
        .collect();

    let (_low_points, risk_levels) = low_points(&board);

    println!("{:?}", risk_levels); //458
}

pub fn part2() {
    let input: String = fs::read_to_string("data/day9.txt").unwrap();
    let re = Regex::new(r"\d").unwrap();

    let board: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            let line: Vec<i32> = re
                .captures_iter(line)
                .map(|d| d[0].parse().unwrap())
                .collect();
            line
        })
        .collect();

    let (low_points, _risk_levels) = low_points(&board);

    let mut basins = Vec::new();

    for &point in low_points.iter() {
        let mut basin: HashSet<(i32, i32)> = HashSet::new();
        basin.insert(point);

        loop {
            let mut new_points: HashSet<(i32, i32)> = HashSet::new();
            basin.iter().for_each(|&point| {
                let x = point.0 as usize;
                let y = point.1 as usize;
                let height = board[x][y];

                neighbors(point)
                    .iter()
                    .filter(|&candidate| basin.get(candidate) == None)
                    .filter(|&candidate| {
                        let h = board[candidate.0 as usize][candidate.1 as usize];
                        h < 9 && height < h
                    })
                    .for_each(|&new_point| {
                        new_points.insert(new_point);
                    });
            });

            basin.extend(&new_points);

            if new_points.is_empty() {
                break;
            }
        }

        basins.push(basin.len());
    }

    basins.sort_unstable();
    basins.reverse();

    let result = basins[0] * basins[1] * basins[2];
    println!("{:?}", result);
}
