pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != 0 {
                nums[i] = nums[j];
                i += 1;
            }
        }

        for k in i..nums.len() {
            nums[k] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }
}
