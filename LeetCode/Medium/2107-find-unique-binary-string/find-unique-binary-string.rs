use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n: usize = nums.len();
        let strings: HashSet<String> = nums.into_iter().collect();
        let mut ans: String = String::new();

        Self::build_and_find_unique_string(String::new(), n, &mut ans, &strings);

        ans
    }

    fn build_and_find_unique_string(s: String, n: usize, ans: &mut String, strings: &HashSet<String>) {
        if s.len() == n {
            if !strings.contains(&s) {
                *ans = s;
            }
            return;
        }

        for c in ['0', '1'] {
            let mut new_s: String = s.clone();
            new_s.push(c);
            Self::build_and_find_unique_string(new_s, n, ans, strings);
        }
    }

}