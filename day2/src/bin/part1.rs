use lib::*;

#[derive(Debug, Clone)]
struct Cube {
    amount: i32,
    color: String,
}

impl Cube {
    fn parse_from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let a = parts.first().expect("exists").parse::<i32>().expect("is number");
        let c = parts.last().expect("msg").to_string();
        Cube {
            amount: a,
            color: c,
        }
    }
}

#[derive(Debug, Clone)]
struct Set {
    cubes: Vec<Cube>,
}

impl Set {
    fn parse_from_input(input: &str) -> Self {
        let cubes: Vec<Cube> = input
            .split(",")
            .map(|f| Cube::parse_from_input(f))
            .collect();

        Set { cubes: cubes }
    }
}

fn main() {
    let lines = read_lines("part1.txt");

    let asd = lines
        .into_iter()
        .map(|f| {
            let a: Vec<&str> = f.split(":").collect();
            let game_id = a
                .first()
                .expect("to esist")
                .split_whitespace()
                .collect::<Vec<&str>>()
                .last()
                .expect("to exist")
                .parse::<i32>().expect("is number");
            // dbg!(game_id);

            let game = a.last().expect("to exist");
            // dbg!(game);

            let sets: Vec<Set> = a
                .last()
                .expect("exists")
                .split(";")
                .map(|x| Set::parse_from_input(x))
                .collect();
            // dbg!(sets.to_vec());

            let r = sets.into_iter().all(|x| {
                x.cubes.into_iter().all(|cube| {
                    let c = cube.color.as_str();
                    // dbg!(cube.clone());
                    let n = match c {
                        "red" => 12,
                        "green" => 13,
                        "blue" => 14,
                        _ => panic!()
                    };
                    // dbg!(n);
                    cube.amount <= n                    
                })
            });

            // println!("{:?}", r);
            if r {
                Some(game_id)
            } else {
                None
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect::<Vec<i32>>();

    println!("{:?}", asd);

    let s: i32 = asd.into_iter().sum();

    println!("Sum: {}", s);
}
