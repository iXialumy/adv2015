use std::fs;

pub fn day01() {
    let input = fs::read_to_string("inputs/01.txt").unwrap();
    let values: Vec<i32> = input.chars().map(|c| if c == '(' { 1 } else { -1 }).collect();
    let a: i32 = values.iter().sum();
    let b = values
        .iter()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1;
    println!("Day01: a: {a}, b: {b:?}")
}
