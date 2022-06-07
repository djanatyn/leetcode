use std::iter::{repeat, FromIterator};

pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let characters: Vec<char> = s.chars().collect();
    let mut output: Vec<String> = Vec::new();

    for chunk in characters.chunks(k as usize) {
        let mut result = String::new();
        result += &String::from_iter(chunk);

        let chunk_length = chunk.len();
        if chunk_length != k as usize {
            let rest = repeat(fill).take(k as usize - chunk_length);
            result += &String::from_iter(rest);
        }

        output.push(result);
    }

    output
}

fn main() {
    dbg!(divide_string("abcdefghi".to_string(), 3, 'x'));
    dbg!(divide_string("abcdefghij".to_string(), 3, 'x'));
}
