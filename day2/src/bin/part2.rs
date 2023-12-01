use lib::*;

fn string_to_number(s: &str) -> String {
    match s {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => s.parse().expect("to be number"),
    }
}

fn find_string_number(s: &str) -> Vec<(usize, String)> {
    let v = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    v.into_iter()
        .map(|f| (s.match_indices(f).collect::<Vec<(usize, &str)>>()))
        .filter(|f| f.len() > 0)
        .flatten()
        .map(|(x, y)| (x, string_to_number(y)))
        .collect()
}

fn main() {
    let sv = read_lines("part1.txt");

    let nv: Vec<String> = sv
        .into_iter()
        .map(|x| {
            let mut tn = find_string_number(x.as_str());
            tn.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));

            vec![
                (*tn.first().expect("exists")).clone().1,
                (*tn.last().expect("exists")).clone().1,
            ]
            .join("")
        })
        .collect();

    let sum: i32 = nv.iter().map(|f| f.parse::<i32>().expect("to work")).sum();
    println!("Sum: {}", sum) //54504
}
