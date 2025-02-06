use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut answer: i32 = 0;
        let mut map = HashMap::<i32, i32>::new();

        for (i, vi) in nums.iter().enumerate() {
            for (_, vj) in nums.iter().enumerate().skip(i + 1) {
                let product: i32 = vi * vj;
                *map.entry(product).or_insert(0) += 1;
            }
        }

        for v in map.values() {
            if *v > 1 {
                answer += *v * (*v - 1) / 2;
            }
        }

        answer * 8
    }
}