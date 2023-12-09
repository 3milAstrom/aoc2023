use lib::*;

pub fn parse_numbers(input: &str) -> Vec<i64> {
    input.split(" ").map(|c| {
        c.parse::<i64>().expect("to be valid number")
    }).collect()
}

fn inter_exterpolate(vec: &Vec<i64>) -> i64 {
    let mut v: Vec<i64> = Vec::with_capacity(vec.len() - 1);
    for i in 0..(vec.len()) {
        if i != vec.len() - 1 {
            v.push(vec[i+1] - vec[i]);
        }
    }

    if !(v.iter().filter(|c| **c == 0).count() == v.len()) {
        let ans = inter_exterpolate(&v);
        let current = v.last().expect("to be number");
        return current + ans
    };
    v.last().expect("to be a zero").clone()
}

fn main() {
    let lines = read_lines("part1.txt");
    let r = lines.iter().map(|l| parse_numbers(l.as_str())).collect::<Vec<Vec<i64>>>();

    let sum: i64 = r.iter().map(|l| {
        let ans = inter_exterpolate(l);
        l.last().expect("to exist") + ans
    }).sum();

    println!("Sum: {}", sum) // 1842168671

}
