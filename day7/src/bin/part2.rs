use std::{cmp::Ordering, collections::HashMap, fs, iter::zip};

use lib::*;

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    value: u32,
    hand_type: u32,
}

fn n_kind(n: u32, h: Vec<u32>) -> bool {
    // dbg!(n, h.clone());

    let mut map: HashMap<u32, u32> = HashMap::new();

    let n_j = h.clone().iter().filter(|c| **c == 1).count() as u32;
    if n_j == n {
        return true;
    }

    h.iter().filter(|c| **c != 1).for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    let (_, v) = map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();

    (*v + n_j) == n
}

fn is_house(h: Vec<u32>) -> bool {
    let mut map: HashMap<u32, u32> = HashMap::new();

    let n_j = h.clone().iter().filter(|c| **c == 1).count() as u32;

    h.iter().filter(|c| **c != 1).for_each(|c| {
        if map.contains_key(c) {
            *map.get_mut(c).unwrap() += 1;
        } else {
            map.insert(*c, 1);
        }
    });

    if map.len() == 2 && n_j == 0{
        let v: Vec<u32> = map.iter().map(|f| *f.1).collect();
        return (v[0] == 2 && v[1] == 3) || (v[0] == 3 && v[1] == 2);
    } 
    map.len() == 2 && n_j == 1
}

// fn is_two_pair(h: Vec<u32>) -> bool {
//     let mut map: HashMap<u32, u32> = HashMap::new();

//     let n_j = h.iter().filter(|c| **c == 1).count() as i32;

//     h.iter().filter(|c| **c != 1).for_each(|c| {
//         if map.contains_key(c) {
//             *map.get_mut(c).unwrap() += 1;
//         } else {
//             map.insert(*c, 1);
//         }
//     });

//     if map.len() == 3 {
//         let v: usize = map.iter().map(|f| *f.1).filter(|&f| f == 2).count();
//         return v == 2;
//     } else if map.len() == 4 && n_j == 1 {
//         return true;
//     }

//     false
// }

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

    let value: u32 = if n_kind(5, asd.clone()) {
        0
    } else if n_kind(4, asd.clone()) {
        1
    } else if is_house(asd.clone()) {
        2
    } else if n_kind(3, asd.clone()) {
        3
    } else if is_two_pair(asd.clone()) {
        4
    } else if n_kind(2, asd.clone()) {
        5
    } else if is_distinct(asd.clone()) {
        6
    } else {
        7
    };

    // let n_joker:i32 = hand.iter().filter(|f| **f==1).count() as i32;
    // dbg!(n_joker);

    // joker_rule(value, n_joker)
    value
}

fn char_to_card(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => c
            .to_string()
            .parse::<u32>()
            .expect("To be valud card number"),
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

    hands.sort_by(|a, b| {
        let a_hand_value = a.hand_type;
        let b_hand_value = b.hand_type;
        let c = &a_hand_value.cmp(&b_hand_value);
        match *c {
            std::cmp::Ordering::Equal => apa(a.cards.clone(), b.cards.clone()),
            e => e,
        }
    });
    hands.reverse();

    let sum: u32 = hands
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, h)| ((i + 1) as u32) * h.value)
        .sum();

    println!("Sum: {}", sum);
}
