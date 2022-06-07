#![feature(slice_group_by)]

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let results: Vec<&[String]> = strs
        .group_by(|a, b| {
            let mut a_chars = a.chars().collect::<Vec<_>>().clone();
            let mut b_chars = b.chars().collect::<Vec<_>>().clone();

            a_chars.sort();
            b_chars.sort();

            a_chars == b_chars
        })
        .collect();

    let mut output = Vec::new();
    for group in results {
        output.push(group.to_vec());
    }

    output
}

fn main() {
    let strs: Vec<String> = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let result = group_anagrams(strs);

    println!("{result:?}");
}
