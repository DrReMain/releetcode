use std::{cell::RefCell, rc::Rc};

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

    pub fn from(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree(&vec, 0)
    }

    fn build_tree(vec: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&Some(val)) = vec.get(index) {
            let mut node = TreeNode::new(val);
            let left_child = Self::build_tree(vec, 2 * index + 1);
            let right_child = Self::build_tree(vec, 2 * index + 2);

            node.left = left_child.clone();
            node.right = right_child.clone();

            Some(Rc::new(RefCell::new(node)))
        } else {
            None
        }
    }
}
