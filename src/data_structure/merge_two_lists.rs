// Definition for single-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        listnode1: Option<Box<ListNode>>,
        listnode2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (listnode1, listnode2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val <= node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists(n, Some(node1));
                    Some(node2)
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn it_works() {
        let mut node11 = ListNode::new(1);
        let mut node12 = ListNode::new(2);
        let node13 = ListNode::new(4);

        node12.next = Some(Box::new(node13));
        node11.next = Some(Box::new(node12));

        let mut node21 = ListNode::new(1);
        let mut node22 = ListNode::new(3);
        let node23 = ListNode::new(4);

        node22.next = Some(Box::new(node23));
        node21.next = Some(Box::new(node22));

        let result = Solution::merge_two_lists(Some(Box::new(node11)), Some(Box::new(node21)));
        let node1 = result.as_ref().unwrap();
        let node2 = node1.next.as_ref().unwrap();
        let node3 = node2.next.as_ref().unwrap();
        let node4 = node3.next.as_ref().unwrap();
        let node5 = node4.next.as_ref().unwrap();
        let node6 = node5.next.as_ref().unwrap();
        assert_eq!(node1.val, 1);
        assert_eq!(node2.val, 1);
        assert_eq!(node3.val, 2);
        assert_eq!(node4.val, 3);
        assert_eq!(node5.val, 4);
        assert_eq!(node6.val, 4);
    }
}
