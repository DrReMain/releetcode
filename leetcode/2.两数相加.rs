/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut l1 = l1;
        let mut l2 = l2;
        let mut curr = &mut dummy_head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let n1 = if let Some(node) = l1 {
                l1 = node.next;
                node.val
            } else {
                0
            };
            let n2 = if let Some(node) = l2 {
                l2 = node.next;
                node.val
            } else {
                0
            };

            let sum = carry + n1 + n2;
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
        }

        if carry > 0 {
            curr.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
    }
}
// @lc code=end
