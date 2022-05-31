// i would use a regex but i only have my standard library: s/(\w)\1//
// this is serial

pub fn remove_duplicates(s: String) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let length = s.len();
    let mut result = String::new();

    let mut skip: bool = false;
    let mut duplicates: bool = false;

    for (n, char) in chars.iter().enumerate() {
        // if we need to skip, skip
        if skip {
            skip = false;
            continue;
        }

        // if we are at the end, push and exit
        if n + 1 == length {
            result.push(*char);
            break;
        }

        // otherwise, look at the next element
        let comparison = chars[n + 1];

        // if we match the next character, continue *and* skip next iteration
        // we mark that we have seen a duplicate
        if *char == comparison {
            skip = true;
            duplicates = true;
            continue;
        }

        // if we do not have a duplicate, push the element
        result.push(*char);
    }

    // if we found a duplicate, check the string again
    if duplicates {
        return remove_duplicates(result);
    }

    // otherwise, return the result
    result
}

fn main() {
    dbg!(remove_duplicates("abbaca".to_string()));
}
