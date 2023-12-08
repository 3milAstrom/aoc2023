use std::collections::HashMap;
use num::integer::lcm;
use lib::*;

fn find_cycle_len(a: String, instruction: Vec<bool>, map: HashMap<String,(String, String)>) -> u64 {
    let mut current =  a.clone();
    let mut found = false;
    let mut i = 0;
    let mut it = instruction.into_iter().cycle();

    while !found {
        let right = it.next().expect("").clone();
        let (l, r) = map.get(&current).unwrap();

        if (right && r.ends_with("Z")) || (!right && l.ends_with("Z")){
            found = !found;
        } else if right {
            current = r.clone();
        } else if !right {
            current = l.clone();
        }
        i += 1;
    }
    i
}


fn main() {
    let lines = read_lines("part1.txt");
    let instruction = &lines[0].chars().map(|c| {
        match c {
            'L' => false,
            'R' => true,
            n => panic!("{}", n)
        }
    }).collect::<Vec<bool>>();

    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let keys: Vec<String> = lines.iter().skip(2).map(|l| {
        let r = l.replace(" ", "");
        let split1 = r.split("=").map(|c| c.to_string()).collect::<Vec<String>>();
        let key = split1[0].clone();
        let p2: Vec<String> = split1[1].rm_f_l().clone().split(",").map(|c| c.to_string()).collect();
        map.insert(key.clone(), (p2[0].to_string(), p2[1].to_string()));
        key
    }).collect();

    let starts: Vec<String> = keys.clone().into_iter().filter(|c| c.ends_with("A")).collect();

    let l = starts.iter().map(|f| {
        find_cycle_len(f.clone(), instruction.clone(), map.clone())
    }).fold(1, |acc, length| {
        lcm(acc, length) as u64
    });

    println!("{}", l);

}
