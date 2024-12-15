use aoc_24::day_one::{prepare, sum_distances};

fn main() {
    // Day 1 Part One
    let input = prepare();
    let result = sum_distances(input);
    println!("{:?}", result);
}
