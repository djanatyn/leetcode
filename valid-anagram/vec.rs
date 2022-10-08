// https://leetcode.com/submissions/detail/817389489/
//
// Runtime: 52 ms
// Memory Usage: 2.8 MB

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut one = s.chars().collect::<Vec<_>>();
        let mut two = t.chars().collect::<Vec<_>>();

        one.sort();
        two.sort();

        println!("{:?}", one);
        println!("{:?}", two);

        one == two
    }
}
