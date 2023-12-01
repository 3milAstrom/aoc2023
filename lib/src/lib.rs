use std::fs;

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).expect("Should ba able to read file")
}

pub fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn is_number(s: String) -> bool {
    let r = s.parse::<i32>();
    match r {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn char_is_number(s: &char) -> bool {
    is_number(s.to_string())
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
