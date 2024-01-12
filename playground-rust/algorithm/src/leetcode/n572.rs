use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(node) = root {
            Self::check(Some(node.clone()), sub_root.clone())
                || Self::is_subtree(node.borrow().left.clone(), sub_root.clone())
                || Self::is_subtree(node.borrow().right.clone(), sub_root.clone())
        } else {
            false
        }
    }
    pub fn check(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (t1, t2) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(n1), Some(n2)) => {
                let (b1, b2) = (n1.borrow(), n2.borrow());
                if b1.val == b2.val {
                    Self::check(b1.left.clone(), b2.left.clone())
                        && Self::check(b1.right.clone(), b2.right.clone())
                } else {
                    false
                }
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
