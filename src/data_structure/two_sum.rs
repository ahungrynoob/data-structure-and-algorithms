use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) {
                return vec![map[&complement] as i32, i as i32];
            }

            map.insert(nums[i], i);
        }
        return vec![];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        let result = Solution::two_sum(vec![2, 7, 11, 5], 9);
        assert_eq!(result, vec![0, 1]);
    }
}
