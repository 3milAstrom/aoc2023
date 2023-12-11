use itertools::Itertools;
use lib::*;

fn find_empty_rows(g: &Vec<Vec<String>>) -> Vec<usize> {
    g.iter()
        .enumerate()
        .filter_map(|(row, l)| {
            if l.iter().all(|s| s == ".") {
                Some(row)
            } else {
                None
            }
        })
        .collect()
}

fn find_empty_cols(g: &Vec<Vec<String>>) -> Vec<usize> {
    let n_cols = g[0].len();
    let mut columns = vec![];

    for c in 0..n_cols {
        let mut a = true;
        for r in 0..g.len() {
            let c = &g[r][c];
            if *c != "." {
                a = false;
                break;
            }
        }
        if a {
            columns.push(c);
        }
    }
    columns
}

fn expand_point(p: (usize, usize), row: &Vec<usize>, col: &Vec<usize>) -> (usize, usize) {
    let r = p.0 + (col.iter().filter(|x| **x < p.0).count() * (1000000 - 1));
    let c = p.1 + (row.iter().filter(|y| **y < p.1).count() * (1000000 - 1));

    (r, c)
}

fn m_d(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (((p1.0 as i32) - (p2.0 as i32)).abs() + ((p1.1 as i32) - (p2.1 as i32)).abs()) as usize
}

fn main() {
    let lines = read_lines("part1.txt");
    let galaxy_map = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let empty_rows = find_empty_rows(&galaxy_map);
    let empty_cols = find_empty_cols(&galaxy_map);

    let mut p_of_i: Vec<(usize, usize)> = vec![];

    for r in 0..galaxy_map.len() {
        let row = &galaxy_map[r];
        for c in 0..row.len() {
            let item = &galaxy_map[r][c];

            if *item == "#" {
                let p = expand_point((c, r), &empty_rows, &empty_cols);
                p_of_i.push(p);
            }
        }
    }

    let sum: usize = p_of_i
        .iter()
        .enumerate()
        .map(|(i, p1)| {
            let r = p_of_i
                .iter()
                .enumerate()
                .filter_map(|(j, p2)| if &j > &i { Some(m_d(*p1, *p2)) } else { None })
                .collect::<Vec<usize>>();
            r
        })
        .flatten()
        .sum::<usize>();

    println!("{}", sum); // 613686987427
}
