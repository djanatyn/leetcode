// i would use a regex but i only have my standard library: s/(\w)\1//
// this is serial

pub fn remove_duplicates(s: String) -> String {
    let mut result = String::new();

    for char in s.chars() {
        if let Some(last) = result.chars().last() {
            if char == last {
                result.pop().expect("failed to remove char");
                continue;
            }
        }

        result.push(char);
    }

    result
}

fn main() {
    dbg!(remove_duplicates("abbaca".to_string()));
}
