use std::fs;

// --- Day 25: Sea Cucumber ---

// 47.88ms in release
pub fn part1() {
    let trench = fs::read_to_string("data/day25.txt").unwrap();
    // Can use as_bytes() instead of chars() here, would require 1
    // byte instead of 4, however, performance remains unchanged for
    // this specific example.
    let mut board: Vec<Vec<char>> = trench.lines().map(|s| s.chars().collect()).collect();
    let mut steps = 0;
    let width = board[0].len();
    let height = board.len();

    let empty = '.';
    let right = '>';
    let down = 'v';

    let mut changed = true;
    while changed {
        steps += 1;

        changed = false;
        for l in board.iter_mut() {
            let last: char = l[width - 1];
            let prev = l.clone();
            for i in 0..l.len() - 1 {
                // Could use (i + 1 % width) here, but its almost 2x slower
                if prev[i + 1] == empty && prev[i] == right {
                    l[i + 1] = right;
                    l[i] = empty;
                    changed = true;
                }
            }
            if last == right && prev[0] == empty {
                l[width - 1] = empty;
                l[0] = right;
                changed = true;
            }
        }

        for j in 0..width {
            let last: char = board[height - 1][j];

            let mut prev: Vec<char> = vec![0 as char; height];
            for k in 0..height {
                prev[k] = board[k][j];
            }

            for k in 0..height - 1 {
                if prev[k + 1] == empty && prev[k] == down {
                    board[k][j] = empty;
                    board[k + 1][j] = down;
                    changed = true;
                }
            }
            if last == down && prev[0] == empty {
                board[height - 1][j] = empty;
                board[0][j] = down;
                changed = true;
            }
        }

        if !changed {
            println!("{:?}", steps);
            break;
        }
    }
}
