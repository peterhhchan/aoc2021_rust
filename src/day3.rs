use std::fs;
use std::str;

fn vec_to_int(v: Vec<u8>) -> isize {
    let g = str::from_utf8(&v).unwrap();
    isize::from_str_radix(g, 2).unwrap()
}

pub fn part1() {
    let input = fs::read_to_string("data/day3.txt").unwrap();
    let readings: Vec<_> = input.lines().map(|s| s.as_bytes()).collect();

    let num_bits = readings[0].len();
    let mut freqs = vec![0; num_bits];
    for n in readings.iter() {
        for b in 0..num_bits {
            if n[b] == 49 {
                freqs[b] += 1;
            }
        }
    }

    let gamma: Vec<u8> = freqs
        .iter()
        .map(|&b| if b > readings.len() - b { b'1' } else { b'0' })
        .collect();
    // We can also do:
    // epsilon = ((1 << Num_bits) - 1) ^ gamma
    let epsilon: Vec<u8> = freqs
        .iter()
        .map(|&b| if b < readings.len() - b { b'1' } else { b'0' })
        .collect();

    println!("{}", vec_to_int(gamma) * vec_to_int(epsilon));
}

fn most_common(readings: &Vec<&[u8]>, idx: usize) -> u8 {
    let num_readings = readings.len();
    let freq = readings
        .iter()
        .fold(0, |sum, n| if n[idx] == b'1' { sum + 1 } else { sum });
    if freq >= num_readings as usize - freq {
        b'1'
    } else {
        b'0'
    }
}

pub fn part2() {
    let input = fs::read_to_string("data/day3.txt").unwrap();
    let mut o2_readings: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();

    let mut idx = 0;
    while o2_readings.len() > 1 {
        let bit = most_common(&o2_readings, idx);
        o2_readings = o2_readings.into_iter().filter(|&s| s[idx] == bit).collect();
        idx += 1;
    }

    let mut co2_readings: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
    idx = 0;
    while co2_readings.len() > 1 {
        let bit = most_common(&co2_readings, idx);
        co2_readings = co2_readings
            .into_iter()
            .filter(|&s| s[idx] != bit)
            .collect();
        idx += 1;
    }

    println!(
        "{:?}",
        vec_to_int(o2_readings[0].to_vec()) * vec_to_int(co2_readings[0].to_vec())
    );
}
