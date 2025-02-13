use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut small_nums: BinaryHeap<Reverse<i64>> = BinaryHeap::from(
            nums.into_iter()
                .map(|v| Reverse(v as i64))
                .collect::<Vec<_>>(),
        );
        let mut count: i32 = 0;

        while small_nums.len() > 1 {
            let x = small_nums.pop().unwrap().0;
            let y = small_nums.pop().unwrap().0;
            if x >= k as i64 {
                break;
            }
            let sum: i64 = min(x, y) * 2 + max(x, y);

            small_nums.push(Reverse(sum));

            count += 1;
        }

        count
    }

}