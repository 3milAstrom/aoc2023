use lib::*;

fn main() {
    let sv = read_lines("part2.txt");

    let nv: Vec<String> = sv
        .into_iter()
        .map(|x| {
            let c: Vec<char> = x.chars().collect();
            let n: Vec<char> = c.into_iter()
                .filter(|y| char_is_number(y))
                .collect();
            vec![*n.first().expect("to exist"), *n.last().expect("to exist")].iter().collect()
        })
        .collect();
    let sum: i32 = nv.iter().map(|f| f.parse::<i32>().expect("to be numbers")).sum();

    println!("Sum: {}", sum) //54597
}
