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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        inorder_recursive(root, &mut result);
        result
    }
}

fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder_recursive(node.borrow().left.clone(), result);
            result.push(node.borrow().val);
            inorder_recursive(node.borrow().right.clone(), result);
        }
        None => return,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut root = TreeNode::new(1);
        let root_right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root_right.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.right = root_right;

        let result = Solution::inorder_traversal(Some(Rc::new(RefCell::new(root))));

        assert_eq!(result, vec![1, 3, 2]);
    }
}
