// https://leetcode.com/submissions/detail/817401616/
//
// Runtime: 7 ms
// Memory Usage: 2.3 MB

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut seen: HashMap<char, i64> = HashMap::new();

        for c in s.chars() {
            if let Some(i) = seen.get_mut(&c) {
                *i += 1;
            } else {
                seen.insert(c, 1);
            }
        }

        for c in t.chars() {
            if let Some(i) = seen.get_mut(&c) {
                *i -= 1;
            } else {
                seen.insert(c, -1);
            }
        }

        for balance in seen.values() {
            if *balance != 0 {
                return false;
            }
        }

        return true;
    }
}
