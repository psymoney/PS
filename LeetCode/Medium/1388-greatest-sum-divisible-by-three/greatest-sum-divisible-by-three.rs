impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, i32::MIN, i32::MIN];

        for &n in nums.iter() {
            let prev = dp.clone();

            for prev_val in prev {
                let local_sum = n + prev_val;
                let remainder = local_sum.rem_euclid(3) as usize;
                dp[remainder] = dp[remainder].max(local_sum);
            }
        }
        dp[0] as i32
    }
}