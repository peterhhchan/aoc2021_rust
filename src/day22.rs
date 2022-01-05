use regex::Regex;
use std::cmp;
use std::convert::TryInto;
use std::fs;

fn into_array(cube: &[i64]) -> [i64; 6] {
    cube.try_into().expect("")
}

fn overlap(start1: i64, end1: i64, start2: i64, end2: i64) -> (i64, i64) {
    (cmp::max(start1, start2), cmp::min(end1, end2))
}

pub fn sum(cubes: &Vec<[i64; 6]>) -> i64 {
    //println!("Cubes: {:?}", cubes);
    cubes.iter().fold(0, |sum, cube| {
        sum + (1 + cube[1] - cube[0]) * (1 + cube[3] - cube[2]) * (1 + cube[5] - cube[4])
    })
}

pub fn part1() {
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
    commands.iter().take(20).for_each(|(is_on, cube)| {
        let mut off_additions = Vec::new();
        let mut on_additions = Vec::new();

        on.iter().for_each(|c2: &[i64; 6]| {
            let (a, b) = overlap(cube[0], cube[1], c2[0], c2[1]);
            let (c, d) = overlap(cube[2], cube[3], c2[2], c2[3]);
            let (e, f) = overlap(cube[4], cube[5], c2[4], c2[5]);
            if b >= a && d >= c && f >= e {
                off_additions.push([a, b, c, d, e, f]);
            }
        });

        off.iter().for_each(|c2: &[i64; 6]| {
            let (a, b) = overlap(cube[0], cube[1], c2[0], c2[1]);
            let (c, d) = overlap(cube[2], cube[3], c2[2], c2[3]);
            let (e, f) = overlap(cube[4], cube[5], c2[4], c2[5]);
            if b >= a && d >= c && f >= e {
                on_additions.push([a, b, c, d, e, f]);
            }
        });

        on.append(&mut on_additions);
        off.append(&mut off_additions);
        if *is_on {
            on.push(into_array(&cube[..]));
        }
    });

    let total_on = sum(&on) - sum(&off);
    //  println!("{:?}", on);
    //  println!("{:?}", off);
    println!("{:?}", total_on);
}
