use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut nums: Vec<i32> = nums.to_owned();
        nums.sort();
        if nums.len() % 2 != 0 {
            return false;
        }
        for i in (1..nums.len()).step_by(2) {
            if nums[i - 1] != nums[i] {
                return false;
            }
        }
        true
    }
}