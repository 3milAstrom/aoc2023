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
        let mut current = pattern[index].clone();
        let mut prev = pattern[index - 1].clone();

        for r_i in 0..prev.len() {
            let su = if prev[r_i] == "#" {
                prev[r_i] = ".".to_string();
                ".".to_string()
            } else {
                prev[r_i] = "#".to_string();
                "#".to_string()
            };

            if current == prev {
                let mut new_pat = pattern.clone();
                new_pat[index - 1][r_i] = su;
                println!("{:?}", new_pat);
                let middle = (index - 1, index);
                let ans = get_remaining_horizontal(pattern, middle);
                if ans > 0 {
                    return ans;
                }
            }
        }

    }
    0
}


fn find_vertical_smudge(pattern: &Vec<Vec<String>>) -> usize {
    for column in 1..pattern[0].len() {
        let mut current: Vec<String> = vec![];
        let mut prev: Vec<String> = vec![];

        for row in pattern {
            current.push(row[column].clone());
            prev.push(row[column - 1].clone());
        }

        for c_i in 0..prev.len() {
            let su = if prev[c_i] == "#" {
                prev[c_i] = ".".to_string();
                ".".to_string()
            } else {
                prev[c_i] = "#".to_string();
                "#".to_string()
            };

            if current == prev {
                let mut new_pat = pattern.clone();
                new_pat[c_i][column - 1] = su;
                println!("{:?}", new_pat);
                let ans = get_remaining_vertical(&new_pat, (column -1, column));
                if ans > 0 {
                    return ans;
                }
            }
        }
    }

    0
}


fn main() {
    let file = read_file("part2.txt");

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
            let horizontal: usize = find_horizontal(pattern);
            println!("");
            let vertical = find_vertical_smudge(pattern);

            // println!("{:?}", horizontal);
            println!("{:?}", vertical);

            (horizontal * 100) + vertical
        })
        .sum::<usize>();

    println!("{}", sum);
}
