use std::{cmp::min, fs};
use itertools::Itertools;

pub fn day02() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();
    let a = day02a(&input);
    let b = day02b(&input);
    
    println!("Day 02 a: {a}, b: {b}")
}

fn day02a(input: &str) -> i32 {
    let presents = input.lines();

    presents.map(|present| to_present_dimensions(present))
        .map(paper_needed)
        .sum()
}

fn day02b(input: &str) -> i32 {
    let presents = input.lines();

    presents.map(|present| to_present_dimensions(present))
        .map(ribbon_needed)
        .sum()
}

fn paper_needed((l,w,h): (i32, i32, i32)) -> i32 {
    let s1 = l * w;
    let s2 = w * h;
    let s3 = h * l;

    let surface_area = 2 * (s1 + s2 + s3);
    let extra = min(s1, min(s2, s3));
    surface_area + extra
}

fn ribbon_needed((l,w,h): (i32, i32, i32)) -> i32 {
    let mut lens = vec![l,w,h];
    lens.sort();
    let len = 2*(lens[0] + lens[1]);
    let bow: i32 = lens.iter().product();
    len + bow
}

fn to_present_dimensions(line: &str) -> (i32, i32, i32) {
    line.split("x")
    .map(|x| x.parse::<i32>().unwrap())
    .next_tuple()
    .unwrap()
}