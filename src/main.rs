use std::time::Instant;

//mod day1;
//mod day2;
//mod day4;
//mod day5;
//mod day6;
//mod day8;
//mod day9;
mod day25;

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
    // day8::part2();
    //day9::part1();
    //day9::part2();

    day25::part1();

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
