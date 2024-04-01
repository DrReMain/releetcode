use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(n) = root {
            let n = n.borrow();
            Solution::is_symmetric_check(n.left.clone(), n.right.clone())
        } else {
            true
        }
    }
    pub fn is_symmetric_check(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(l), Some(r)) => {
                let (l, r) = (l.borrow(), r.borrow());
                l.val == r.val
                    && Solution::is_symmetric_check(l.left.clone(), r.right.clone())
                    && Solution::is_symmetric_check(l.right.clone(), r.left.clone())
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
    fn test_is_symmetric1() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        let expected = true;
        assert_eq!(Solution::is_symmetric(root), expected);
    }

    #[test]
    fn test_is_symmetric2() {
        let root = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        let expected = false;
        assert_eq!(Solution::is_symmetric(root), expected);
    }

    #[bench]
    fn bench_is_symmetric(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        b.iter(|| Solution::is_symmetric(root.clone()));
    }
}
