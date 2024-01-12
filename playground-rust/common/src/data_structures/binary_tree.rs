use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T = i32> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl<T> TreeNode<T> {
    #[inline]
    pub fn new (val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}
