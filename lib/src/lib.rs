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

pub trait Utils {
    fn rm_f_l(&self) -> Self;
}

impl Utils for String {
    fn rm_f_l(&self) -> Self {
        let mut chars: std::str::Chars<'_> = self.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string()
    }

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
