use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

use lib::*;

use std::str::FromStr;

use ndarray::arr2;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Tile_type {
    Vertical_pipe,
    Horizontal_pipe,
    Ne_bend,
    Nw_bend,
    Sw_bend,
    Se_bend,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
struct TileTypeError;

impl FromStr for Tile_type {
    type Err = TileTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile_type::Vertical_pipe),
            "-" => Ok(Tile_type::Horizontal_pipe),
            "L" => Ok(Tile_type::Ne_bend),
            "J" => Ok(Tile_type::Nw_bend),
            "7" => Ok(Tile_type::Sw_bend),
            "F" => Ok(Tile_type::Se_bend),
            "." => Ok(Tile_type::Ground),
            "S" => Ok(Tile_type::Start),
            x => panic!("Panic got {}", x),
        }
    }
}

fn can_walk(c: &Tile_type, n: &Tile_type, dir: u8) -> bool {
    if *n == Tile_type::Ground {
        return false;
    }

    match c {
        Tile_type::Start => *n != Tile_type::Ground,
        Tile_type::Vertical_pipe => match dir {
            0 | 1 => false,
            2 => {
                *n == Tile_type::Sw_bend
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Start
            }
            3 => {
                *n == Tile_type::Ne_bend
                    || *n == Tile_type::Nw_bend
                    || *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Start
            }
            _ => panic!(),
        },
        Tile_type::Horizontal_pipe => match dir {
            2 | 3 => false,
            0 => {
                *n == Tile_type::Ne_bend
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Start
            }
            1 => {
                *n == Tile_type::Nw_bend
                    || *n == Tile_type::Sw_bend
                    || *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Start
            }
            _ => panic!(),
        },
        Tile_type::Ne_bend => match dir {
            1 => {
                *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Nw_bend
                    || *n == Tile_type::Sw_bend
                    || *n == Tile_type::Start
            }
            2 => {
                *n == Tile_type::Sw_bend
                    || *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Start
            }
            0 | 3 => false,
            _ => panic!(),
        },
        Tile_type::Nw_bend => match dir {
            1 | 3 => false,
            0 => {
                *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Ne_bend
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Start
            }
            2 => {
                *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Sw_bend
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Start
            }
            _ => panic!(),
        },
        Tile_type::Sw_bend => match dir {
            1 | 2 => false,
            0 => {
                *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Ne_bend
                    || *n == Tile_type::Se_bend
                    || *n == Tile_type::Start
            }
            3 => {
                *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Ne_bend
                    || *n == Tile_type::Nw_bend
                    || *n == Tile_type::Start
            }
            _ => panic!(""),
        },
        Tile_type::Se_bend => match dir {
            0 | 2 => false,
            1 => {
                *n == Tile_type::Horizontal_pipe
                    || *n == Tile_type::Sw_bend
                    || *n == Tile_type::Nw_bend
                    || *n == Tile_type::Start
            }
            3 => {
                *n == Tile_type::Vertical_pipe
                    || *n == Tile_type::Ne_bend
                    || *n == Tile_type::Nw_bend
                    || *n == Tile_type::Start
            }
            _ => panic!(),
        },

        _ => panic!("unexpected"),
    }
}

fn get_adjc_points(x: usize, y: usize, dim: (usize, usize)) -> Vec<Option<(usize, usize)>> {
    let mut n: Vec<Option<(usize, usize)>> = vec![];

    if x > 0 {
        n.push(Some((x - 1, y)));
    } else {
        n.push(None)
    }

    if x < dim.0 - 1 {
        n.push(Some((x + 1, y)))
    } else {
        n.push(None)
    }

    if y > 0 {
        n.push(Some((x, y - 1)))
    } else {
        n.push(None)
    }

    if y < dim.1 - 1 {
        n.push(Some((x, y + 1)))
    } else {
        n.push(None)
    }

    n
}

fn get_oposite(x: usize) -> usize {
    match x {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        usize::MAX => x,
        _ => panic!(),
    }
}

fn shoelace(v: Vec<(usize, usize)>) -> i64 {
    let mut asd: i64 = 0;
    for ((x, y), (x1, y1)) in v.into_iter().tuple_windows() {
        asd += (y as i64 + y1 as i64) * (x as i64 - x1 as i64);
    }

    asd

}

fn main() {
    let lines = read_lines("part1.txt");
    let mut map: HashMap<(usize, usize), Tile_type> = HashMap::new();
    let mut start = (0, 0);
    let dim = (lines[0].len(), lines.len());

    lines.iter().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            let t = c
                .to_string()
                .parse::<Tile_type>()
                .expect("To be valid type");
            if t == Tile_type::Start {
                start = (col, row);
            }

            map.insert((col, row), t);
        });
    });

    let mut c = (start.0, start.1 + 1);
    let mut i = 1;

    let mut done = false;
    let mut direction = 3;
    let mut visited = vec![c];

    while !done {
        let current_tile = map.get(&c).unwrap();
        let adj = get_adjc_points(c.0, c.1, dim)
            .iter()
            .enumerate()
            .map(|(j, x)| {
                if j == get_oposite(direction) {
                    return (x, false, j);
                }
                let a = match x {
                    Some(v) => {
                        let t = map.get(v).unwrap();
                        can_walk(current_tile, t, j as u8)
                    }
                    None => false,
                };
                (x, a, j)
            })
            .filter(|(x, y, _)| *y)
            .map(|(x, _, j)| (x.unwrap(), j))
            .collect::<Vec<((usize, usize), usize)>>();

        if adj.iter().filter(|(x, _)| *x == start).count() == 1 {
            done = true;
        }

        direction = adj[0].1;
        c = adj[0].0;
        visited.push(c);
        i += 1;
    }

    println!("{}", (shoelace(visited) / 2).abs() - (i / 2) + 1); // 356
}
