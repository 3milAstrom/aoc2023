use std::fs;

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Should ba able to read file")
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_hello() -> () {
    println!("Hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
