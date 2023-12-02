use lib::*;
use nom::{
    character::complete::{char, digit1, alpha0, alphanumeric0},
    combinator::{map, map_res},
    sequence::separated_pair,
    IResult,
    bytes::complete::tag, multi::{many0, separated_list0}
};
use std::str::FromStr;

pub fn parse_numbers(input: &str) -> IResult<&str, i32> {
    map_res(digit1, i32::from_str)(input)
}


#[derive(Debug, Clone)]
struct Cube {
    amount: i32,
    color: String,
}

impl Cube {
    fn parse(s: &str) -> IResult<&str, Self> {
        let parse_parts = separated_pair(parse_numbers, char(' '), alpha0);
        map(parse_parts, |(x,y)| Cube{amount: x ,color: y.to_string() })(s)
    }
}

#[derive(Debug, Clone)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn parse(s: &str) -> IResult<&str, Self> {
        let parsed_cubes = separated_list0(tag(", "), Cube::parse);
        map(parsed_cubes, |x| Set{cubes: x})(s)
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl Game {
    fn parse(s: &str) -> IResult<&str, Self> {
        let parse_sets = separated_list0(tag("; "), Set::parse);
        let parse_game = separated_pair(alpha0, char(' '), parse_numbers);
        let parse_parts = separated_pair(parse_game, tag(": "), parse_sets);
        map(parse_parts, |((_, id), sets)| Game{id, sets} )(s)
    }
}

fn main() {
    let lines = read_lines("part1.txt");
    let games: Vec<Game> = lines.into_iter().map(|line| {
        Game::parse(line.as_str()).expect("").1
    }).collect();

    let s: i32 = games.into_iter().map(|game| {
        let mut r = i32::MIN;
        let mut g = i32::MIN;
        let mut b = i32::MIN;

        game.sets.into_iter().for_each(|set| {
            set.cubes.into_iter().for_each(|cube| {
                let c = cube.color.as_str();
                let a = cube.amount;
                if c == "red" && a > r {
                    r = a;
                } else if c == "green" && a > g {
                    g = a;
                } else if c == "blue" && a > b {
                    b = a;
                }
            })
        });
        r*g*b

    }).sum();

    println!("Sum: {}", s); //70265
}
