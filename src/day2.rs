use std::fs;

pub fn part1() -> i32 {
    let readings = fs::read_to_string("data/day2.txt").unwrap();

    let res = readings
        .lines()
        .map(|s| {
            let mut line = s.split(' ');
            (line.next().unwrap(), line.next().unwrap().parse().unwrap())
        })
        .fold(
            (0, 0),
            |(h, d), (direction, v): (&str, i32)| match direction {
                "forward" => (h + v, d),
                "down" => (h, d - v),
                "up" => (h, d + v),
                _ => (h, d),
            },
        );

    println! {"{:?}", res.0 * res.1};
    res.0 * res.1
}

pub fn part2() -> i32 {
    let readings = fs::read_to_string("data/day2.txt").unwrap();

    let res = readings
        .lines()
        .map(|s| {
            let mut line = s.split(' ');
            (line.next().unwrap(), line.next().unwrap().parse().unwrap())
        })
        .fold(
            (0, 0, 0),
            |(h, d, aim), (direction, v): (&str, i32)| match direction {
                "forward" => (h + v, d + aim * v, aim),
                "down" => (h, d, aim - v),
                "up" => (h, d, aim + v),
                _ => (h, d, aim),
            },
        );

    println! {"{:?}", res.0 * res.1};
    res.0 * res.1
}
