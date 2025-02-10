impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut chars = Vec::new();

        for c in s.chars() {
            if c.is_digit(10) {
                chars.pop();
            } else {
                chars.push(c);
            }
        }

        chars.iter().collect()
    }
}