use core::time;
use std::thread::{self, sleep};

use itertools::Itertools;
use lib::*;

fn to_number_vec(s: String) -> Vec<i64> {
    s.split(",")
        .map(|s| s.parse::<i64>().expect("To be valid number"))
        .collect()
}

fn search(
    spring: &Vec<String>,
    groups: &Vec<usize>,
    spring_index: usize,
    group_index: usize,
    count: usize,
) -> usize {
    let mut l_index_groups = group_index.clone();

    if spring_index == spring.len() {
        return 1;
    }
    // println!();
    // println!("{:?}", spring);
    // println!("{}, {}", spring_index, group_index);

    let mut local_count = count;
    // let acc = 0;

    // println!("---- acc: {} -----", acc);
    for index in spring_index..spring.len() {
        if l_index_groups >= groups.len() {
            if spring[index..spring.len()]
                .iter()
                .all(|s| **s == ".".to_string())
            {
                // println!("Success empty end");
                return 1;
            }
        }

        if l_index_groups >= groups.len() {
            if spring[index..spring.len()]
                .iter()
                .any(|s| **s == "#".to_string())
            {
                // println!("falied empty end");
                return 0;
            }
        }

        let s = &spring[index];
        // println!("{} {} {} {}", index, l_index_groups, local_count, s);

        if *s == "#".to_string() {
            local_count += 1;
            // if index == spring.len() - 1 && local_count == groups[l_index_groups] && l_index_groups >= groups.len(){
            //     println!("Success #");
            //     return 1;
            // } else
            if local_count > groups[l_index_groups] {
                // println!("Fail #");
                return 0;
            }
        }

        if *s == ".".to_string() {
            if l_index_groups < groups.len() && local_count == groups[l_index_groups] {
                l_index_groups += 1;
                local_count = 0;
            } else if local_count > 0 {
                // println!("Fail .");
                return 0;
            }
        }

        if *s == "?".to_string() {
            let mut s1 = spring.clone();
            let mut s2 = spring.clone();

            s1[index] = ".".to_string();
            s2[index] = "#".to_string();

            // println!("Branch 1");
            let p1 = search(&s1, groups, index, l_index_groups, local_count);
            // let p2 = if local_count != groups[l_index_groups]{
            //     search(&s2, groups, index, l_index_groups, local_count)
            // } else { 0 };

            // println!("Branch 2");
            let p2 = search(&s2, groups, index, l_index_groups, local_count);
            // println!("=========");

            // if p1 == 0 && p2 == 0 {
            //     return acc;
            // }

            // println!("acc: {}", acc);
            // println!("Returned {} {}", p1, p2);
            return p1 + p2;
        }
    }

    if l_index_groups >= (groups.len()){
        // println!("Success");
        return 1;
    }

    if local_count == groups[l_index_groups] && l_index_groups == groups.len() - 1{
        // println!("Success 2");
        return 1;

    }
    // println!("Fail {} {}", local_count, l_index_groups );
    0
}

fn main() {
    let mut lines = read_lines("part1.txt")
        .iter()
        .map(|f| {
            let p = f.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            let s = p[0].chars().map(|s| s.to_string()).collect::<Vec<String>>();
            let n = p[1]
                .split(",")
                .map(|x| x.parse::<usize>().expect(""))
                .collect::<Vec<usize>>();

            (s, n)
        })
        .collect::<Vec<(Vec<String>, Vec<usize>)>>();

    // lines = vec![lines[5].clone()];

    let asd = lines
        .iter()
        .map(|(l, v)| {
            let vv: Vec<String> = l
                .iter()
                .skip_while(|s| **s == ".".to_string())
                .map(|x| x.clone())
                .collect();
            // println!("{:?}", vv);
            let a = search(&vv, v, 0, 0, 0);
            // dbg!(a);
            a
        })
        .collect::<Vec<usize>>();

    // asd.iter().for_each(|f| println!("{}", f));

    let sum = asd.iter().sum::<usize>();

    println!("Sum {}", sum);
}
