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
            let sets: Vec<Set> = a
                .last()
                .expect("exists")
                .split(";")
                .map(|x| Set::parse_from_input(x))
                .collect();

            let mut r = i32::MIN;
            let mut g = i32::MIN;
            let mut b = i32::MIN;

            sets.into_iter().for_each(|x| {
                x.cubes.into_iter().for_each(|cube| {
                    let c = cube.color.as_str();
                    let a = cube.amount;
                    // dbg!(cube.clone());
                    if c == "red" && a > r {
                        r = a;
                    } else if c == "green" && a > g {
                        g = a;
                    } else if c == "blue" && a > b {
                        b = a;
                    }
                })
            });
            
            r*g*b            
        })
        .collect::<Vec<i32>>();

    println!("{:?}", asd);

    let s: i32 = asd.into_iter().sum();

    println!("Sum: {}", s);
}
