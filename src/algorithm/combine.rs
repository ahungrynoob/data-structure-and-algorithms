pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if n < k {
            return vec![];
        }

        let mut vecs = Vec::new();
        let mut vec = Vec::new();
        backtrack(&mut vecs, &mut vec, n, k, 1);
        return vecs;
    }
}

fn backtrack(vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, n: i32, k: i32, start: usize) {
    if vec.len() == k as usize {
        vecs.push(vec.clone());
        return;
    }

    let mut i = start;
    while i <= (n - (k - vec.len() as i32) + 1) as usize {
        vec.push(i as i32);
        backtrack(vecs, vec, n, k, i + 1);
        vec.pop();
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
    }
}
