use lib::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, digit1},
    combinator::{map, map_res},
    multi::{separated_list0},
    sequence::separated_pair,
    IResult,
};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}

pub fn get_numbers(input: &str) -> IResult<&str, Vec<i32>> {
    let parts = separated_list0(tag(" "), parse_numbers);
    map(parts, |v| v)(input)
}

#[derive(Debug)]
struct Card {
    winning: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        let t = separated_pair(alpha0, tag(" "), parse_numbers);
        let p = separated_pair(get_numbers, tag(" | "), get_numbers);
        let parse_parts = separated_pair(t, tag(": "), p);
        map(parse_parts, |(_, (w, n))| Card {
            winning: w.clone(),
            numbers: n.clone(),
        })(input)
    }

    fn get_score(&self) -> u32 {
        let count: u32 = (self
            .numbers
            .iter()
            .filter(|&&n| self.winning.iter().any(|&w| w == n))
            .count()) as u32;

        if count > 0 {
            (2 as u32).pow(count - 1)
        } else {
            0 as u32
        }
    }
}

fn main() {
    let lines = read_lines("part1.txt");
    let s: u32 = lines
        .into_iter()
        .map(|line| {
            let card = Card::parse(line.as_str()).expect("To be valid card").1;
            card.get_score()
        })
        .sum();

    println!("{:?}", s); //24542
}
