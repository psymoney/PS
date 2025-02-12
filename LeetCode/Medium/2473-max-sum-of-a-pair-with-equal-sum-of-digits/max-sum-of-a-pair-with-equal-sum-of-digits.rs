use std::cmp;
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut max: i32 = -1;

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
                .and_modify(|vec| {
                    vec[0] += 1;
                    let local_max: usize = vec.iter().skip(1).map(|v| *v + num as usize).max().unwrap();
                    max = cmp::max(max, local_max as i32);
                    vec.push(num as usize);
                })
                .or_insert(vec![1, num as usize]);
        }

        max
    }
}