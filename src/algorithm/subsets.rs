pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 0 {
            return vec![];
        }

        let mut vecs = Vec::new();
        let mut vec = Vec::new();
        backtrack(&mut vecs, &mut vec, &nums, 0);
        vecs
    }
}

fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
    vecs.push(vec.clone());

    for i in start..nums.len() {
        vec.push(nums[i]);
        backtrack(vecs, vec, nums, i + 1);
        vec.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
    }
}
