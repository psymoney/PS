use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut s: String = String::new();

        for (i, bs) in nums.into_iter().enumerate() {
            let c = bs.chars().nth(i).unwrap();
            if c == '0' {
                s.push('1');
            } else {
                s.push('0');
            }
        }

        s
    }
}