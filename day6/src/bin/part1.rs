use std::{fs, iter::zip};

use lib::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, alphanumeric0, digit1, not_line_ending},
    combinator::{map, map_res, rest},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str,  i32> {
    map_res(digit1,  i32::from_str)(input)
}

pub fn get_numbers(input: &str) -> IResult<&str, Vec< i32>> {
    let parts = separated_list0(tag(" "), parse_numbers);
    map(parts, |v| v)(input)
}

pub fn get_parts(l: Vec<String>) -> Vec<(i32,i32)> {
    let parts: Vec<Vec<i32>> = l.into_iter().map(|s| {
        let asd = s.split(": ").collect::<Vec<&str>>();
        let a = get_numbers(asd[1]).expect("to be valid numbers").1;
        a
    }).collect();

    let a = parts[0].clone();
    let b = parts[1].clone();

    zip(a, b).collect::<Vec<(i32,i32)>>()
}

fn get_number_valid_strategies(pair: (i32,i32)) -> i32 {
    let length = pair.1;
    let time = pair.0;
    let s = (0..time).collect::<Vec<i32>>();

    s.into_iter().filter(|i| {
        
        if *i == 1 {
            println!("{} {}",*i, 6 >= length);
            return 6 >= length
        }
        let speed = *i;
        let remaining = time - i;
        let distance = remaining * speed;

        let a = distance > length;

        println!("{} {} {} {}",*i, remaining, distance, a);
        a
    }).count() as i32
}

fn main() {
    let lines = read_lines("part1.txt");
    let p = get_parts(lines);
    
    let v: i32 = p.into_iter().map(|f| {
        get_number_valid_strategies(f)
    }).fold(1,|acc, c| {
        acc * c
    });


    dbg!(v); //633080

}
