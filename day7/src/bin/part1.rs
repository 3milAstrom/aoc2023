use std::{fs, iter::zip, collections::HashMap, cmp::Ordering};

use lib::*;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    value: u32,
    hand_type: u32,
}

fn fiok(h: Vec<u32>) -> bool {
    let f = h[0];
    h.iter().all(|&c| c==f)
}

fn n_kind(n: u32, h: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    h.iter().for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    let (_, v) = map.iter().max_by(|a,b| {
        a.1.cmp(b.1)
    }).unwrap();

    *v == n
}

fn is_house(h: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    h.iter().for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    if map.len() == 2 {
        let v: Vec<u32> = map.iter().map(|f| *f.1).collect();
        return (v[0] == 2 && v[1] == 3) || (v[0] == 3 && v[1] == 2)
    }

    false
}

fn is_two_pair(h: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    h.iter().for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    if map.len() == 3 {
        let v: usize = map.iter().map(|f| *f.1).filter(|&f| f == 2).count();
        return v == 2
    }

    false
}

fn is_distinct(h: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    h.iter().for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    map.len() == 5
}

fn evaluate_hand(hand: Vec<u32>) -> u32 {
    let asd = hand.clone();

    if fiok(asd.clone()) {
        return 0
    }

    if n_kind(4, asd.clone()) {
        return 1
    }

    if is_house(asd.clone()) {
        return 2;
    }

    if n_kind(3, asd.clone()) {
        return 3;
    }

    if is_two_pair(asd.clone()) {
        return 4;
    }

    if n_kind(2, asd.clone()) {
        return 5;
    }

    if is_distinct(asd.clone()) {
        return 6;
    }

    7
}

fn char_to_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_string().parse::<u32>().expect("To be valud card number")
    }
}

fn apa(a: Vec<u32>, b: Vec<u32>) -> Ordering {


    for i in 0..5 {
        let c1 = a[i].clone();
        let c2 = b[i].clone();

        let o = c2.cmp(&c1);
        
        if o == Ordering::Greater || o == Ordering::Less {
            return o;
        }
    }

    // Ordering::Equal
    panic!("equal")
}


fn main() {
    let lines = read_lines("part1.txt");
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|f| {
            let parts = f.split(" ").collect::<Vec<&str>>();
            let hand = parts[0];
            let value = parts[1].parse::<u32>().expect("to be number");
            let cards = hand.chars().map(|c| char_to_card(c)).collect::<Vec<u32>>();
            let rank = evaluate_hand(cards.clone());

            Hand {
                cards,
                hand_type: rank,
                value,
            }
        })
        .collect();

    hands.sort_by(|a,b| {
        let a_hand_value = a.hand_type;
        let b_hand_value = b.hand_type;
        let c = &a_hand_value.cmp(&b_hand_value);
        match *c {
            std::cmp::Ordering::Equal => apa(a.cards.clone(), b.cards.clone()),
            e => e
        }
    });
    hands.reverse();

    
    let sum: u32 = hands.clone().into_iter().enumerate().map(|(i,h)| {
        ((i + 1) as u32) * h.value
    }).sum();

    println!("Sum: {}", sum);
}
