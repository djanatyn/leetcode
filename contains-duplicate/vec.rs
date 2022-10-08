// https://leetcode.com/submissions/detail/817373496/
//
// Runtime: 714 ms
// Memory Usage: 2.9 MB

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = Vec::new();

        for num in nums {
            // This operation is O(n).
            if seen.contains(&num) {
                return true;
            } else {
                seen.push(num);
            }
        }

        return false;
    }
}
