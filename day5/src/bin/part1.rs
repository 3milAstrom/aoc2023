use std::fs;

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

pub fn parse_numbers(input: &str) -> IResult<&str, i64> {
    map_res(digit1, i64::from_str)(input)
}

pub fn get_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    let parts = separated_list0(tag(" "), parse_numbers);
    map(parts, |v| v)(input)
}

pub fn get_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    let s = separated_pair(alphanumeric0, tag(": "), get_numbers);
    map(s, |(_, v)| v)(input)
}

#[derive(Debug, Clone)]
struct SMap {
    ds: i64,
    ss: i64,
    rl: i64,
}

impl SMap {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(get_numbers, |f| {
            dbg!(f.clone());
            SMap {
                ds: f[0],
                ss: f[1],
                rl: f[2],
            }
        })(input)
    }
}

fn get_maps(input: &str) -> IResult<&str, Vec<SMap>> {
    let s = separated_list0(tag("\n"), SMap::parse);
    let t = separated_pair(not_line_ending, tag("\n"), s);
    map(t, |(_, v)| v)(input)
}

fn get_location(seed: i64, map: Vec<SMap>) -> i64 {
    let asd = map.iter().find(|f| seed >= f.ss && seed <= f.ss + f.rl);
    // dbg!(asd);

    let a = if asd.is_some() {
        let s = asd.unwrap();
        let d = seed - s.ss;
        s.ds + d
    } else {
        seed
    };

    a
}

fn main() {
    let contents = fs::read_to_string("part1.txt").expect("Should have been able to read the file");

    let parts: Vec<&str> = contents.split("\n\n").collect();

    let seeds = get_seeds(parts[0]).expect("to be valid numbers").1;
    let sts = get_maps(parts[1]).expect("to be valid sts").1;
    let stf = get_maps(parts[2]).expect("to be valid stf").1;
    let ftw = get_maps(parts[3]).expect("to be valid ftw").1;
    let wtl = get_maps(parts[4]).expect("to be valid wtl").1;
    let ltt = get_maps(parts[5]).expect("to be valid ltt").1;
    let tth = get_maps(parts[6]).expect("to be valid tth").1;
    let htl = get_maps(parts[7]).expect("to be valid htl").1;

    let v = vec![sts, stf, ftw, wtl, ltt, tth, htl];

    let sum: i64 = seeds
        .into_iter()
        .map(|s| {
            v.iter().fold(s, |a, c| get_location(a, c.clone()))
        })
        .min()
        .expect("");

    dbg!(sum); //214922730
    // dbg!(sts);
}
