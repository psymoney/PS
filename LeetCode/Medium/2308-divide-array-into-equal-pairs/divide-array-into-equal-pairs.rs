use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut counter_map: HashMap<i32, usize> = HashMap::new();

        for n in nums {
            let mut count: &mut usize = counter_map.entry(n).or_insert(0);
            if *count == 1 {
                *count -= 1;
            } else if *count == 0 {
                *count += 1;
            }
        }

        for c in counter_map.values() {
            if *c != 0 {
                return false;
            }
        }

        true
    }
}