pub struct Solution;

impl Solution {
    pub fn climb_stairs_dp(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        let mut dp: Vec<i32> = vec![0; (n + 1) as usize];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..(n + 1) as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }

        return dp[n as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs_dp(2), 2);
        assert_eq!(Solution::climb_stairs_dp(4), 5);
    }
}
