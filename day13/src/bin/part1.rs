use lib::*;

fn get_remaining_horizontal(pattern: &Vec<Vec<String>>, middle: (usize, usize)) -> usize {
    let done = false;
    let mut upp = middle.0;
    let mut down = middle.1;
    let mut indexes: Vec<(usize,usize)> = vec![];

    while !done {

        let upp_v = &pattern[upp];
        let down_v = &pattern[down];

        if upp_v == down_v {
            indexes.push((upp + 1, down + 1));
        } else {
            return 0;
        }

        if ((upp as i32) - 1) == -1 || down + 1 == pattern.len(){
            break;
        }

        upp -= 1;
        down += 1;

    }

    middle.0 + 1
    // indexes
}

fn get_remaining_vertical(pattern: &Vec<Vec<String>>, middle: (usize, usize)) -> usize {
    let mut left = middle.0;
    let mut right = middle.1;

    let mut done = false;

    let mut indexes: Vec<(usize,usize)> = vec![];

    while !done {
        let mut left_v: Vec<String> = vec![];
        let mut right_v: Vec<String> = vec![];
        for row in pattern {
            left_v.push(row[left].clone());
            right_v.push(row[right].clone());
        }

        if left_v == right_v {
            indexes.push((left + 1, right + 1));
        } else {
            return 0
        }

        if (left as i32) - 1 == -1 || right + 1 == pattern[0].len() {
            break;
        }

        left -= 1;
        right += 1;
    }

    middle.0 + 1
}

fn find_horizontal(pattern: &Vec<Vec<String>>) -> usize{
    for index in 1..pattern.len() {
        let current = &pattern[index];
        let prev = &pattern[index - 1];

        if current == prev {
            let middle = (index - 1, index);
            let ans = get_remaining_horizontal(pattern, middle);
            if ans > 0 {
                return ans;
            }
        }
    }
    0
}



fn find_vertical(pattern: &Vec<Vec<String>>) -> usize {
    for column in 1..pattern[0].len() {
        let mut current: Vec<String> = vec![];
        let mut prev: Vec<String> = vec![];

        for row in pattern {
            current.push(row[column].clone());
            prev.push(row[column - 1].clone());
        }

        if current == prev {
            let ans = get_remaining_vertical(pattern, (column -1, column));
            if ans > 0 {
                return ans;
            }
        }
    }

    0
}

fn main() {
    let file = read_file("part1.txt");

    let a: Vec<&str> = file.split("\n\n").collect();

    let patterns: Vec<Vec<Vec<String>>> = a
        .iter()
        .map(|s| {
            let line: Vec<Vec<String>> = s
                .split("\n")
                .map(|l| {
                    let signs: Vec<String> = l.chars().map(|f| f.to_string()).collect();
                    signs
                })
                .collect();
            line
        })
        .collect();

    // println!("{:?}", patterns);

    let sum = patterns
        .iter()
        .map(|pattern| {
            let horizontal = find_horizontal(pattern);
            let vertical = find_vertical(pattern);

            // println!("{:?}", horizontal);
            // println!("{:?}", vertical);

            (horizontal * 100) + vertical
        })
        .sum::<usize>();

    println!("{}", sum);
}
