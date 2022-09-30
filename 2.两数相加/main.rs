// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None; // 头节点，依靠tail创建第一个node，需要给tail可变引用，所以head也需要mut
        let mut tail = &mut head;
        let mut t = (l1, l2, 0, 0);

        loop {
            t = match t {
                // 如果l1和l2都为None，就是都遍历完了，并且进位值为0，不需要创建新节点直接break
                (None, None, _, 0) => break,
                // 如果进位值存在，则把它换到t.2的位置去创建最后一个节点，并且在下次循环break
                (None, None, _, c) => (None, None, c, 0),
                // 如果l1和l2其中一个为None，则把另一个指向next
                (Some(list), None, _, c) | (None, Some(list), _, c) => {
                    (list.next, None, (list.val + c) % 10, (list.val + c) / 10)
                }
                // 如果l1和l2都不为None，则把它俩都指向next
                (Some(l1), Some(l2), _, c) => (
                    l1.next,
                    l2.next,
                    (l1.val + l2.val + c) % 10,
                    (l1.val + l2.val + c) / 10,
                ),
            };

            // 从当前位的值来创建一个节点
            *tail = Some(Box::new(ListNode::new(t.2)));

            // tail是 &mut Option<Box<ListNode>> 类型
            // 需要先转换为 Option<&mut Box<ListNode>>
            // 后unwrap() 才能得到 &mut Box<ListNode>
            // 然后通过 Deref trait，将tail指向它的next
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}
