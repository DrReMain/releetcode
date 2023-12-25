/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // match (list1, list2) {
        //     (Some(mut node1), Some(mut node2)) => {
        //         if node1.val < node2.val {
        //             let n = node1.next.take();
        //             node1.next = Solution::merge_two_lists(n, Some(node2));
        //             Some(node1)
        //         } else {
        //             let n = node2.next.take();
        //             node2.next = Solution::merge_two_lists(Some(node1), n);
        //             Some(node2)
        //         }
        //     }
        //     (Some(node), None) | (None, Some(node)) => Some(node),
        //     _ => None,
        // }
        let mut list1 = list1;
        let mut list2 = list2;
        let mut dummy_head = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_head;
        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }
        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy_head.next
    }
}
// @lc code=end
