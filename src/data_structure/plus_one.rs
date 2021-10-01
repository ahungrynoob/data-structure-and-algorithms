pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;
        loop {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }

            if i > 0 {
                digits[i] = 0;
                i -= 1;
            } else {
                break;
            }
        }

        digits = vec![0; digits.len() + 1];
        digits[0] = 1;
        return digits;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let result1 = Solution::plus_one(vec![1, 2, 3]);
        let result2 = Solution::plus_one(vec![9, 9, 9]);
        assert_eq!(result1, vec![1, 2, 4]);
        assert_eq!(result2, vec![1, 0, 0, 0]);
    }
}
