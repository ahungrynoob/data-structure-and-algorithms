pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: Vec<i32> = vec![0; n as usize];

        return recursion(n as usize, &mut memo);
    }
}

fn recursion(n: usize, memo: &mut Vec<i32>) -> i32 {
    if n <= 2 {
        return n as i32;
    }

    if memo[n - 1] == 0 {
        memo[n - 1] = recursion(n - 1, memo);
    }

    if memo[n - 2] == 0 {
        memo[n - 2] = recursion(n - 2, memo);
    }

    return memo[n - 1] + memo[n - 2];
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(4), 5);
    }
}
