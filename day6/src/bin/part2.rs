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

pub fn parse_numbers(input: &str) -> IResult<&str,   i64> {
    map_res(digit1,   i64::from_str)(input)
}

pub fn get_numbers(input: &str) -> IResult<&str, Vec<  i64>> {
    let parts = separated_list0(tag(" "), parse_numbers);
    map(parts, |v| v)(input)
}

pub fn get_parts(l: Vec<String>) -> ( i64, i64) {
    let parts: Vec< i64> = l.into_iter().map(|s| {
        let asd = s.split(": ").collect::<Vec<&str>>();
        let b = asd[1].replace(" ", "");
        parse_numbers(b.as_str()).expect("to be valid numbers").1
    }).collect();

    let a = parts[0].clone();
    let b = parts[1].clone();
    (a,b)
}

fn get_number_valid_strategies(pair: ( i64, i64)) ->  i64 {
    let length = pair.1;
    let time = pair.0;
    let s = (0..time).collect::<Vec< i64>>();

    let asd = s.into_iter().filter(|i| {
        
        if *i == 1 {
            return 6 >= length
        }
        let speed = *i;
        let remaining = time - i;
        let distance = remaining * speed;

        let a = distance > length;
        a
    }).count();

    dbg!(asd.clone());

    asd as  i64
}

fn main() {
    let lines = read_lines("part1.txt");
    let p = get_parts(lines);

    dbg!(p.clone());
    
   let v = get_number_valid_strategies(p);

    dbg!(v); // 20048741

}
