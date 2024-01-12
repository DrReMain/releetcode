#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T = i32> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

