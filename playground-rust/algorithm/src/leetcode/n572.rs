use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                Self::is_subtree_check(Some(a.clone()), Some(b.clone()))
                    || Self::is_subtree(a.borrow().left.clone(), Some(b.clone()))
                    || Self::is_subtree(a.borrow().right.clone(), Some(b.clone()))
            }
        }
    }
    pub fn is_subtree_check(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && Self::is_subtree_check(a.borrow().left.clone(), b.borrow().left.clone())
                    && Self::is_subtree_check(a.borrow().right.clone(), b.borrow().right.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_is_subtree() {
        let root1 = TreeNode::from(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root1 = TreeNode::from(vec![Some(4), Some(1), Some(2)]);
        let result1 = Solution::is_subtree(root1, sub_root1);
        assert_eq!(result1, true);

        let root2 = TreeNode::from(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root2 = TreeNode::from(vec![Some(4), Some(1), Some(2)]);
        let result2 = Solution::is_subtree(root2, sub_root2);
        assert_eq!(result2, false);
    }

    #[bench]
    fn bench_is_subtree(b: &mut Bencher) {
        let root = TreeNode::from(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = TreeNode::from(vec![Some(4), Some(1), Some(2)]);
        b.iter(|| Solution::is_subtree(root.clone(), sub_root.clone()));
    }
}
