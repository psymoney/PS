use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut answer = (nums.len() * (nums.len() - 1) / 2) as i64;
        let mut frequency = HashMap::<i32, usize>::new();

        for (index, value) in nums.iter().enumerate() {
            let key = index as i32 - *value;
            *frequency.entry(key).or_insert(0) += 1;
        }

        for value in frequency.values() {
            if *value > 1 {
                answer -= (*value * (*value - 1) / 2) as i64;
            }
        }

        answer
    }
}