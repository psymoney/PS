use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<usize, (usize, BinaryHeap<i32>)> = HashMap::new();

        for num in nums {
            let mut rem: usize = (num % 10) as usize;
            let mut quo: usize = (num / 10) as usize;
            let mut sum: usize = rem;

            while quo != 0 {
                rem = quo % 10;
                quo = quo / 10;
                sum += rem;
            }

            freq.entry(sum)
                .and_modify(|(f, heap)| {
                    *f += 1;
                    heap.push(num);
                })
                .or_insert((1, BinaryHeap::from(vec![num])));
        }

        freq.values_mut()
            .filter_map(|(f, heap)| {
                if *f > 1 {
                    let first = heap.pop()?;
                    let second = heap.pop()?;
                    Some(first + second)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(-1)
    }

}