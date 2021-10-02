use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..strs.len() {
            let mut chars = vec![];
            for c in strs[i].chars() {
                chars.push(c);
            }
            chars.sort();

            let key: String = chars.into_iter().collect();
            let value = map.get(&key);
            if value.is_some() {
                let mut v = value.unwrap().to_vec();
                v.push(strs[i].clone());
                map.insert(key, v);
            } else {
                map.insert(key, vec![strs[i].clone()]);
            }
        }

        for val in map.values() {
            vecs.push(val.to_vec());
        }

        return vecs;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structure::is_anagram;
    #[test]
    fn it_works() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let result = Solution::group_anagrams(input);
        for v in result.into_iter() {
            if v.len() > 1 {
                assert!(is_anagram::Solution::is_anagram(v[0].clone(), v[1].clone()));
            }
        }
    }
}
