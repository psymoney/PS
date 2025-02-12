use std::cmp;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max = [0; 82];
        let mut answer: i32 = -1;

        for num in nums {
            let mut rem: usize = (num % 10) as usize;
            let mut quo: usize = (num / 10) as usize;
            let mut sum: usize = rem;

            while quo != 0 {
                rem = quo % 10;
                quo = quo / 10;
                sum += rem;
            }

            if (max[sum] != 0) {
                answer = cmp::max(answer, num + max[sum]);
            }
            max[sum] = cmp::max(max[sum], num);
        }

        answer
    }
}