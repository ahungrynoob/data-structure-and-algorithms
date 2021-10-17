use std::{cell::RefCell, rc::Rc};

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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                if node.borrow().left.is_none() {
                    return Self::min_depth(node.borrow().right.clone()) + 1;
                }

                if node.borrow().right.is_none() {
                    return Self::min_depth(node.borrow().right.clone()) + 1;
                }

                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();
                Self::min_depth(left).min(Self::min_depth(right)) + 1
            }
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tree = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        };

        assert_eq!(Solution::min_depth(Some(Rc::new(RefCell::new(tree)))), 2);
    }
}
