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
        let mut s1 = Vec::with_capacity(128);
        let mut s2 = Vec::with_capacity(128);
        let mut cur = 0;

        let mut tmp1 = l1;
        while let Some(b) = tmp1 {
            tmp1 = b.next;
            s1.push(b.val);
        }
        let mut tmp2 = l2;
        while let Some(b) = tmp2 {
            tmp2 = b.next;
            s2.push(b.val);
        }
    }
}
