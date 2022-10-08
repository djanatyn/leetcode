// https://leetcode.com/submissions/detail/817375636/
//
// Runtime: 25 ms
// Memory Usage: 3.6 MB

use std::collections::HashMap;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashMap::new();

        for num in nums {
            // This operation is O(n).
            if let Some(_) = seen.get(&num) {
                return true;
            } else {
                seen.insert(num, true);
            }
        }

        return false;
    }
}
