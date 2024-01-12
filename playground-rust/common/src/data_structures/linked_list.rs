#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(vec: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        let mut current = &mut head;
        for &val in vec.iter().rev() {
            *current = Some(Box::new(ListNode::new(val)));
            current = &mut current.as_mut().unwrap().next;
        }
        head
    }
}
