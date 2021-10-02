use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        for c in s.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
            if *count < 0 {
                return false;
            }
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert!(Solution::is_anagram(
            String::from("anagram"),
            String::from("nagaram")
        ));
        assert!(!Solution::is_anagram(
            String::from("car"),
            String::from("rat")
        ));
    }
}
