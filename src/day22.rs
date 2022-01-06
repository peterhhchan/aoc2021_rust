use regex::Regex;
use std::convert::TryInto;
use std::fs;

// --- Day 22: Reactor Reboot ---

// Approach using positive and negative spaces.  Performance is a bit
// disappointing as the number of cubes becomes very large.

fn into_array(cube: &[i64]) -> [i64; 6] {
    cube.try_into().expect("")
}

fn overlap(start1: i64, end1: i64, start2: i64, end2: i64) -> (i64, i64) {
    (start1.max(start2), end1.min(end2))
}

fn sum(cubes: &[[i64; 6]]) -> i64 {
    cubes.iter().fold(0, |sum, cube| {
        sum + (1 + cube[1] - cube[0]) * (1 + cube[3] - cube[2]) * (1 + cube[5] - cube[4])
    })
}

fn intersections(cubes: &[[i64; 6]], cube: &[i64]) -> Vec<[i64; 6]> {
    cubes
        .iter()
        .fold(Vec::new(), |mut intersect, c2: &[i64; 6]| {
            let (a, b) = overlap(cube[0], cube[1], c2[0], c2[1]);
            let (c, d) = overlap(cube[2], cube[3], c2[2], c2[3]);
            let (e, f) = overlap(cube[4], cube[5], c2[4], c2[5]);
            if b >= a && d >= c && f >= e {
                intersect.push([a, b, c, d, e, f]);
            }
            intersect
        })
}

pub fn reactor(n: usize) {
    let re = Regex::new(r"-*\d+").unwrap();
    let commands = fs::read_to_string("data/day22.txt").unwrap();
    let commands: Vec<(bool, Vec<i64>)> = commands
        .lines()
        .map(|s| {
            let cube: Vec<i64> = re.captures_iter(s).map(|x| x[0].parse().unwrap()).collect();
            let command = s.starts_with("on");
            (command, cube)
        })
        .collect();

    let mut on: Vec<[i64; 6]> = Vec::new();
    let mut off: Vec<[i64; 6]> = Vec::new();
    commands.iter().take(n).for_each(|(is_on, cube)| {
        let off_additions = intersections(&on, cube);
        let on_additions = intersections(&off, cube);

        on.extend(&on_additions);
        off.extend(&off_additions);

        if *is_on {
            on.push(into_array(&cube[..]));
        }
    });

    let total_on = sum(&on) - sum(&off);
    println!("{:?}", on.len());
    println!("{:?}", off.len());

    println!("{:?}", total_on);
}

// 4 ms - 606484
pub fn part1() {
    reactor(20);
}

// 110 ms - 1162571910364852
pub fn part2() {
    reactor(1000);
}
