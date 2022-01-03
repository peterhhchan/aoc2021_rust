use std::fs;
use std::str;

// --- Day 25: Sea Cucumber ---

// 47.88ms in release
pub fn part1() {
    let trench = fs::read_to_string("data/day25.txt").unwrap();
    let mut board: Vec<Vec<u8>> = trench.lines().map(|s| s.as_bytes().to_vec()).collect();
    let mut steps = 0;
    let width = board[0].len();
    let height = board.len();

    let EMPTY = b'.';
    let RIGHT = b'>';
    let DOWN = b'v';

    let mut changed = true;
    while changed {
        steps += 1;

        changed = false;
        for l in board.iter_mut() {
            let last: u8 = l[width - 1];
            let prev = l.clone();
            for i in 0..l.len() - 1 {
                // Could use (i + 1 % width) here, but its almost 2x slower
                if prev[i + 1] == EMPTY && prev[i] == RIGHT {
                    l[i + 1] = RIGHT;
                    l[i] = EMPTY;
                    changed = true;
                }
            }
            if last == RIGHT && prev[0] == EMPTY {
                l[width - 1] = EMPTY;
                l[0] = RIGHT;
                changed = true;
            }
        }

        for j in 0..width {
            let last: u8 = board[height - 1][j];

            let mut prev = vec![0; height];
            for k in 0..height {
                prev[k] = board[k][j];
            }

            for k in 0..height - 1 {
                if prev[k + 1] == EMPTY && prev[k] == DOWN {
                    board[k][j] = EMPTY;
                    board[k + 1][j] = DOWN;
                    changed = true;
                }
            }
            if last == DOWN && prev[0] == EMPTY {
                board[height - 1][j] = EMPTY;
                board[0][j] = DOWN;
                changed = true;
            }
        }

        if !changed {
            println!("{:?}", steps);
            break;
        }
    }
}
