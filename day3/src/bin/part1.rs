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

fn char_vec_to_number(v: Vec<char>) -> u32 {
    v.into_iter().collect::<String>().parse::<u32>().expect("")
}

fn main() {
    let lines = read_lines("part1.txt");
    let number_regex = Regex::new(r"([0-9])").unwrap();
    let mut numbers: Vec<NumberRange> = vec![];
    let mut signs_vec: Vec<Sign> = vec![];

    lines.iter().enumerate().for_each(|(y, line)| {
        let signs: std::str::Chars<'_> = line.chars();
        let mut is_number = false;
        let mut number_vec: Vec<char> = vec![];
        let mut number_range: Vec<u32> = vec![];

        signs.enumerate().for_each(|(x, c)| {
            if !number_regex.is_match(c.to_string().as_str()) {
                if is_number {
                    let number = char_vec_to_number(number_vec.clone());
                    numbers.push(NumberRange {
                        x_range: number_range.clone(),
                        row: y as u32,
                        number: number,
                    });

                    is_number = false;
                    number_vec = vec![];
                    number_range = vec![];
                }

                if c != '.' {
                    signs_vec.push(Sign {
                        x: x as u32,
                        y: y as u32,
                    })
                }
            } else {
                if !is_number {
                    is_number = true;
                }
                number_vec.push(c);
                number_range.push(x as u32);

                if x == line.len() - 1 {
                    let number = char_vec_to_number(number_vec.clone());
                    numbers.push(NumberRange {
                        x_range: number_range.clone(),
                        row: y as u32,
                        number: number,
                    });
                }
            }
        });
    });

    let asd: u32 = numbers
        .clone()
        .into_iter()
        .filter(|n| {
            signs_vec
                .clone()
                .into_iter()
                .any(|sign| is_adjecent(sign.x, sign.y, n.x_range.clone(), n.row))
        })
        .map(|f| f.number)
        .sum::<u32>();

    println!("sum: {}", asd); //509115
}
