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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        insert(&root, val);
        root
    }
}

fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(node) = root {
        let mut n = node.borrow_mut();
        let target = if val > n.val {
            &mut n.right
        } else {
            &mut n.left
        };

        if target.is_some() {
            return insert(target, val);
        }

        *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut root = TreeNode::new(4);
        let root_left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let root_right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root_left.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root_left.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));
        root.left = root_left;
        root.right = root_right;

        let root = Solution::insert_into_bst(Some(Rc::new(RefCell::new(root))), 5);
        let result = root
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .val;
        assert_eq!(result, 5);
    }
}
