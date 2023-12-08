use std::collections::HashMap;

use lib::*;

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


    lines.iter().skip(2).for_each(|l| {
        let r = l.replace(" ", "");
        let split1 = r.split("=").map(|c| c.to_string()).collect::<Vec<String>>();
        let key = split1[0].clone();
        let p2: Vec<String> = split1[1].rm_f_l().clone().split(",").map(|c| c.to_string()).collect();
        map.insert(key.clone(), (p2[0].to_string(), p2[1].to_string()));
    });

    let mut current: String = "AAA".to_string();
    let last: String = "ZZZ".to_string();

    let mut found = false;
    let mut i = 0;
    let mut it = instruction.into_iter().cycle();

    while !found {
        let right = it.next().expect("").clone();
        let (l, r) = map.get(&current).unwrap();

        if (right && *r == last) || (!right && *l == last){
            found = !found;
        } else if right {
            current = r.clone();
        } else if !right {
            current = l.clone();
        }
        i += 1;
    }

    println!("{}", i);

}
