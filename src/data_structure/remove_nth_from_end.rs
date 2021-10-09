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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &slow_p.clone();

        for _ in 0..=n {
            fast_p = &fast_p.as_ref().unwrap().next;
        }

        while fast_p.is_some() {
            fast_p = &fast_p.as_ref().unwrap().next;
            slow_p = &mut slow_p.as_mut().unwrap().next;
        }

        let next = slow_p.as_mut().unwrap().next.as_mut().unwrap().next.take();
        slow_p.as_mut().unwrap().next = next;

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn it_works() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let node5 = ListNode::new(5);

        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        node1.next = Some(Box::new(node2));

        let result = Solution::remove_nth_from_end(Some(Box::new(node1)), 2);

        let node1 = result.as_ref().unwrap();
        let node2 = node1.next.as_ref().unwrap();
        let node3 = node2.next.as_ref().unwrap();
        let node4 = node3.next.as_ref().unwrap();
        assert_eq!(node1.val, 1);
        assert_eq!(node2.val, 2);
        assert_eq!(node3.val, 3);
        assert_eq!(node4.val, 5);
        assert_eq!(node4.next, None);
    }
}
