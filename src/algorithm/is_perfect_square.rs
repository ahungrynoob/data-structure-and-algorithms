pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 0 || num == 1 {
            return true;
        }

        let mut left = 2;
        let mut right = num / 2;
        while left <= right {
            let mid = left + (right - left) / 2;
            let guessed_squared = mid * mid;

            if guessed_squared == num {
                return true;
            } else if guessed_squared > num {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}
