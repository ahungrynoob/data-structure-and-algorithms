pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() == 0 {
            return true;
        }

        let mut stack: Vec<char> = vec![];
        for i in 0..chars.len() {
            let char = chars[i];
            if char == '(' {
                stack.push(')');
            } else if char == '[' {
                stack.push(']');
            } else if char == '{' {
                stack.push('}');
            } else if stack.is_empty() || char != stack.pop().unwrap() {
                return false;
            }
        }

        if stack.is_empty() {
            return true;
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid(String::from("(){}[]")), true);
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }
}
