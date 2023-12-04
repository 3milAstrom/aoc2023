use lib::*;
use regex::Regex;

#[derive(Debug, Clone)]
struct NumberRange {
    x_range: Vec<u32>,
    number: u32,
    row: u32,
}

#[derive(Debug, Clone)]
struct Sign {
    x: u32,
    y: u32,
}

fn is_adjecent(sign_x: u32, sign_y: u32, n_range: Vec<u32>, row: u32) -> bool {
    if sign_y == row && (n_range.contains(&(sign_x - 1)) || n_range.contains(&(sign_x + 1))) {
        return true;
    } else if (sign_y - 1 == row || sign_y + 1 == row)
        && (n_range.contains(&sign_x)
            || n_range.contains(&(sign_x - 1))
            || n_range.contains(&(sign_x + 1)))
    {
        return true;
    }

    false
}

fn main() {
    let lines = read_lines("part1.txt");
    let number_regex = Regex::new(r"[0-9]+").unwrap();
    let symbol_regex = Regex::new(r"[^\w\s\d\.]").unwrap();

    let mut numbers: Vec<NumberRange> = vec![];
    let mut signs: Vec<Sign> = vec![];

    lines.into_iter().enumerate().for_each(|(row, line)| {
        let mut collected_numbers = number_regex.find_iter(line.as_str()).map(|f|{
            let range: Vec<u32> = (f.start() as u32..f.end() as u32).collect();
            let number = f.as_str().parse::<u32>().expect("to be number");
            NumberRange { x_range: range, number, row: row as u32 }

        }).collect::<Vec<NumberRange>>();

        let mut collected_signs = symbol_regex.find_iter(line.as_str()).map(|f| {
            Sign {
                x: f.start() as u32,
                y: row as u32
            }
        }).collect::<Vec<Sign>>();

        numbers.append(&mut collected_numbers);
        signs.append(&mut collected_signs);
    });

    let sum: u32 = signs
        .clone()
        .into_iter()
        .map(|sign| {
            numbers
                .clone()
                .iter()
                .filter(|f| is_adjecent(sign.x, sign.y, f.x_range.clone(), f.row))
                .map(|f| f.number)
                .collect::<Vec<u32>>()
        })
        .filter(|f| f.len() == 2)
        .map(|f| {
            let mut p: u32 = 1;
            for ele in f {
                p *= ele;
            }
            p
        })
        .sum::<u32>();

    println!("sum: {}", sum); // 75220503
}
