use std::ops::{Add, Sub};

use lib::*;

pub fn parse_numbers(input: &str) -> Vec<i64> {
    input
        .split(" ")
        .map(|c| c.parse::<i64>().expect("to be valid number"))
        .collect()
}

trait Polate<T> {
    fn inter_exterpolate(vec: &Vec<T>) -> T;
}

impl<T> Polate<T> for Vec<T>
where
    T: Sub<Output = T> + Add<Output = T> + Default + Copy + Eq + PartialEq,
{
    fn inter_exterpolate(vec: &Vec<T>) -> T {
        let mut v: Vec<T> = Vec::with_capacity(vec.len() - 1);
        for i in 0..(vec.len()) {
            if i != vec.len() - 1 {
                v.push(vec[i + 1] - vec[i]);
            }
        }

        if !(v.iter().filter(|c| **c == Default::default()).count() == v.len()) {
            let ans = Self::inter_exterpolate(&v);
            let current = v.first().expect("to be number");
            return *current - ans;
        };
        v.first().expect("to be a zero").clone()
    }
}

fn main() {
    let lines = read_lines("part1.txt");
    let r = lines
        .iter()
        .map(|l| parse_numbers(l.as_str()))
        .collect::<Vec<Vec<i64>>>();

    let sum: i64 = r
        .iter()
        .map(|l| {
            let ans = Vec::inter_exterpolate(l);
            l.first().expect("to exist") - ans
        })
        .sum();

    println!("Sum: {}", sum) // 903
}
