impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut chars: Vec<char> = Vec::new();

        for c in s.chars() {
            chars.push(c);

            if chars.len() >= part.len() {
                let comp_str: String = chars[chars.len() - part.len()..].iter().collect();

                if comp_str == part {
                    for _ in 0..part.len() {
                        chars.pop();
                    }
                }
            }
        }

        chars.iter().collect()
    }

}