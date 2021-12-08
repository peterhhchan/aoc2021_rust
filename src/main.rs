use std::time::Instant;

//mod day1;
//mod day2;
//mod day4;
//mod day5;
//mod day6;
mod day8;

fn main() {
    let start = Instant::now();
    //    day1::part1();
    //    day1::part2();

    //    day4::winners();

    //    day5::part1();
    //    day5::part2();

    // day6::part1();
    // day6::part2();

    // day8::part1();
    day8::part2();

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
