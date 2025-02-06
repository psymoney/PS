use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut answer: i32 = 0;
        let mut map = HashMap::<i32, i32>::new();
        let mut target_map = HashMap::<i32, i32>::new();

        for (i, vi) in nums.iter().enumerate() {
            for (_, vj) in nums.iter().enumerate().skip(i + 1) {
                let product: i32 = vi * vj;
                map.entry(product)
                    .and_modify(|count| {
                        *count += 1;

                        if *count > 1 {
                            *target_map.entry(product).or_insert(1) += 1;
                        }
                    })
                    .or_insert(1);
            }
        }

        for v in target_map.values() {
            answer += *v * (*v - 1) / 2;
        }

        answer * 8
    }
}