pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n;
        if n < 0 {
            x = 1.0 / x;
            n = -n;
        }

        return fast_pow(x, n);
    }
}

fn fast_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    let half = fast_pow(x, n / 2);
    return if n % 2 == 0 {
        half * half
    } else {
        half * half * x
    };
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
        assert!(Solution::my_pow(2.10000, 3) - 9.26100 <= 0.001);
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    }
}
