use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = vec![];
        if root.is_none() {
            return levels;
        };

        let mut deque = VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            let mut current_level = vec![];

            let level_length = deque.len();
            for _ in 0..level_length {
                let n = deque.pop_front();
                if let Some(Some(node)) = n {
                    current_level.push(node.borrow().val);

                    if node.borrow().left.is_some() {
                        deque.push_back(node.borrow().left.clone());
                    }

                    if node.borrow().right.is_some() {
                        deque.push_back(node.borrow().right.clone());
                    }
                }
            }

            levels.push(current_level);
        }

        levels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut root = TreeNode::new(3);
        let root_left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        let root_right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root_right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(15))));
        root_right.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.left = root_left;
        root.right = root_right;

        let result = Solution::level_order(Some(Rc::new(RefCell::new(root))));

        assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
    }
}
