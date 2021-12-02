use std::fs;

fn data() -> Vec<(String, i32)> {
    fs::read_to_string("data/day2.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut line = s.split(' ');
            (
                line.next().unwrap().to_owned(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn part1() -> i32 {
    let res = data().iter().fold(
        (0, 0),
        |(h, d), (direction, v): &(String, i32)| match direction.as_str() {
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
    let res =
        data().iter().fold(
            (0, 0, 0),
            |(h, d, aim), (command, v): &(String, i32)| match command.as_str() {
                "forward" => (h + v, d + aim * v, aim),
                "down" => (h, d, aim - v),
                "up" => (h, d, aim + v),
                _ => (h, d, aim),
            },
        );

    println! {"{:?}", res.0 * res.1};
    res.0 * res.1
}
