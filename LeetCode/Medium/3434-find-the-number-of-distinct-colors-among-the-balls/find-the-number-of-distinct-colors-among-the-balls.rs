use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut color_count = HashMap::<i32, i32>::new();
        let mut ball_map = HashMap::<i32, i32>::new();
        let mut distinct_color_counter: usize = 0;
        let mut answer = vec![];

        for item in queries {
            let idx = item[0];
            let color = item[1];

            let curr_count = color_count.entry(color).or_insert(0);

            if *curr_count == 0 {
                distinct_color_counter += 1;
            }
            *curr_count += 1;

            let prev_color = ball_map.entry(idx).or_insert(0);
            let prev_count = color_count.entry(*prev_color).or_insert(0);

            if *prev_color != 0 && *prev_count > 0 {
                if *prev_count == 1 {
                    distinct_color_counter -= 1;
                }
                *prev_count -= 1;
            }

            ball_map.insert(idx, color);

            answer.push(distinct_color_counter as i32);
        }

        answer
    }
}