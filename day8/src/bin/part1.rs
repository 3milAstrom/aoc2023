use std::{fs, iter::zip, collections::HashMap, cmp::Ordering};

use lib::*;


fn rem_first_and_last(value: String) -> String {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
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

    // dbg!(instruction.clone());

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    

    let keys: Vec<String> = lines.iter().skip(2).map(|l| {
        let r = l.replace(" ", "");
        let split1 = r.split("=").map(|c| c.to_string()).collect::<Vec<String>>();
        let key = split1[0].clone();
        let p2: Vec<String> = rem_first_and_last(split1[1].clone()).split(",").map(|c| c.to_string()).collect();
        map.insert(key.clone(), (p2[0].to_string(), p2[1].to_string()));
        key
    }).collect();

    let mut current: String = "AAA".to_string(); // keys.first().expect("").clone();
    let last: String = "ZZZ".to_string(); // keys.last().expect("").clone();

    // dbg!(map.clone());

    
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
