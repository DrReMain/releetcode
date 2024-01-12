use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut root = root;
        Self::convert_bst_dfs(&mut sum, &mut root);
        root
    }
    pub fn convert_bst_dfs(sum: &mut i32, root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::convert_bst_dfs(sum, &mut node.borrow_mut().right);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            Self::convert_bst_dfs(sum, &mut node.borrow_mut().left);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_convert_bst1() {
        let root = TreeNode::from(vec![
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ]);
        let result = TreeNode::from(vec![
            Some(30),
            Some(36),
            Some(21),
            Some(36),
            Some(35),
            Some(26),
            Some(15),
            None,
            None,
            None,
            Some(33),
            None,
            None,
            None,
            Some(8),
        ]);
        assert_eq!(Solution::convert_bst(root), result);
    }

    #[test]
    fn test_convert_bst2() {
        let root = TreeNode::from(vec![Some(0), None, Some(1)]);
        let result = TreeNode::from(vec![Some(1), None, Some(1)]);
        assert_eq!(Solution::convert_bst(root), result);
    }

    #[test]
    fn test_convert_bst3() {
        let root = TreeNode::from(vec![Some(1), Some(0), Some(2)]);
        let result = TreeNode::from(vec![Some(3), Some(3), Some(2)]);
        assert_eq!(Solution::convert_bst(root), result);
    }

    #[test]
    fn test_convert_bst4() {
        let root = TreeNode::from(vec![Some(3), Some(2), Some(4), Some(1)]);
        let result = TreeNode::from(vec![Some(7), Some(9), Some(4), Some(10)]);
        assert_eq!(Solution::convert_bst(root), result);
    }

    #[bench]
    fn bench_convert_bst(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(4),
            Some(1),
            Some(6),
            Some(0),
            Some(2),
            Some(5),
            Some(7),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            Some(8),
        ]);
        b.iter(|| Solution::convert_bst(root.clone()));
    }
}
