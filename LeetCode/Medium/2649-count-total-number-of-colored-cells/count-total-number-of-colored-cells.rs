impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        n.pow(2) * 2 - 2 * n + 1
    }
}