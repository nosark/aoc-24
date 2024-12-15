use std::iter::*;

pub const INPUT: &str = include_str!("../input/day_one.txt");

pub fn prepare() -> (Vec<i32>, Vec<i32>) {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in INPUT.lines() {
        let tup = line.split("   ").collect::<Vec<_>>();
        left_list.push(
            tup[0]
                .parse::<i32>()
                .expect("failed to parse i32 during line prep"),
        );
        right_list.push(
            tup[1]
                .parse::<i32>()
                .expect("failed to parse i32 during line prep"),
        );
    }

    left_list.sort();
    right_list.sort();

    (left_list, right_list)
}

pub fn sum_distances(lists: (Vec<i32>, Vec<i32>)) -> i32 {
    let mut distance_sum = 0;
    for i in 0..lists.0.len() {
        distance_sum += (lists.0[i] - lists.1[i]).abs();
    }

    distance_sum
}
