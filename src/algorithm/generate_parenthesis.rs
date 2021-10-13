pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut vec = vec![];
        recursion(&mut vec, 0, 0, n, String::from(""));
        return vec;
    }
}

fn recursion(vec: &mut Vec<String>, left: i32, right: i32, n: i32, s: String) {
    if left == n && right == n {
        return vec.push(s);
    }

    if left < n {
        recursion(vec, left + 1, right, n, format!("{}{}", &s, "("));
    }

    if right < left {
        recursion(vec, left, right + 1, n, format!("{}{}", &s, ")"));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
