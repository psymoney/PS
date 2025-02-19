impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let max_size: usize = (1 << (n as usize - 1)) * 3;

        if k as usize > max_size {
            return String::from("");
        }

        let mut bag: Vec<String> = Vec::with_capacity(max_size);
        make_happy(String::new(), n as usize, &mut bag);
        bag[k as usize - 1].clone()
    }
}
fn make_happy(s: String, n: usize, bag: &mut Vec<String>) {
    if s.len() == n {
        bag.push(s);
        return;
    } 
    for c in ['a', 'b', 'c'] {
        if !s.is_empty() && c == s.chars().last().unwrap() {
            continue;
        }
        let mut new_s = s.clone();
        new_s.push(c);
        make_happy(new_s, n, bag);
    }
}