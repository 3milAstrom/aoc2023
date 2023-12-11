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

fn expand_point(p: (usize, usize), row: &Vec<usize>, col: &Vec<usize>) -> Point<i32> {
    let r = p.0 + (col.iter().filter(|x| **x < p.0).count());
    let c = p.1 + (row.iter().filter(|y| **y < p.1).count());
    Point::new_from_tuple((r as i32, c as i32))
}

fn main() {
    let lines = read_lines("part1.txt");
    let galaxy_map = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let empty_rows = find_empty_rows(&galaxy_map);
    let empty_cols = find_empty_cols(&galaxy_map);

    let mut p_of_i: Vec<Point<i32>> = vec![];

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

    let sum: i32 = p_of_i
        .iter()
        .enumerate()
        .map(|(i, p1)| {
            let r = p_of_i
                .iter()
                .enumerate()
                .filter_map(|(j, p2)| if &j > &i { Some(p1.manhattan_distance(*p2)) } else { None })
                .collect::<Vec<i32>>();
            r
        })
        .flatten()
        .sum::<i32>();

    println!("{}", sum); // 9214785
}
