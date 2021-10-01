pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i = 0;
        for j in 0..nums.len() {
            if nums[i] != nums[j] {
                if j - i > 1 {
                    nums[i + 1] = nums[j];
                }
                i += 1;
            }
        }
        return (i + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let mut input0 = vec![];
        let result0 = Solution::remove_duplicates(&mut input0);
        assert_eq!(result0, 0);
        assert_eq!(input0, vec![]);

        let mut input1 = vec![1, 1, 2];
        let result1 = Solution::remove_duplicates(&mut input1);
        assert_eq!(result1, 2);
        assert_eq!(input1, vec![1, 2, 2]);

        let mut input2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let result2 = Solution::remove_duplicates(&mut input2);
        assert_eq!(result2, 5);
        assert_eq!(input2, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
    }
}
