use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = root {
            {
                let mut n = n.borrow_mut();
                (n.left, n.right) = (
                    Self::invert_tree(n.right.take()),
                    Self::invert_tree(n.left.take()),
                );
            }
            Some(n)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_invert_tree1() {
        let root = TreeNode::from(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let result = TreeNode::from(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(root), result);
    }

    #[test]
    fn test_invert_tree2() {
        let root = TreeNode::from(vec![Some(2), Some(1), Some(3)]);
        let result = TreeNode::from(vec![Some(2), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(root), result);
    }

    #[test]
    fn test_invert_tree3() {
        let root = TreeNode::from(vec![]);
        let result = TreeNode::from(vec![]);
        assert_eq!(Solution::invert_tree(root), result);
    }

    #[bench]
    fn bench_convert_bst(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        b.iter(|| Solution::invert_tree(root.clone()));
    }
}
