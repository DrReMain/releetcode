use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let mut count = 0;
            Self::is_unival_subtrees(&mut count, Some(r.clone()), r.borrow().val);
            count
        } else {
            0
        }
    }
    pub fn is_unival_subtrees(
        count: &mut i32,
        node: Option<Rc<RefCell<TreeNode>>>,
        value: i32,
    ) -> bool {
        if let Some(n) = node {
            let n = n.borrow();
            let left = Self::is_unival_subtrees(count, n.left.clone(), n.val);
            let right = Self::is_unival_subtrees(count, n.right.clone(), n.val);

            match (left, right) {
                (true, true) => {
                    *count += 1;
                    n.val == value
                }
                _ => false,
            }
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_count_unival_subtrees1() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(1),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(5),
        ]);
        assert_eq!(Solution::count_unival_subtrees(root), 4);
    }

    #[test]
    fn test_count_unival_subtrees2() {
        let root = TreeNode::from(vec![]);
        assert_eq!(Solution::count_unival_subtrees(root), 0);
    }

    #[test]
    fn test_count_unival_subtrees3() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(5),
        ]);
        assert_eq!(Solution::count_unival_subtrees(root), 6);
    }

    #[bench]
    fn bench_count_unival_subtrees(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(5),
            Some(1),
            Some(5),
            Some(5),
            Some(5),
            None,
            Some(5),
        ]);
        b.iter(|| Solution::count_unival_subtrees(root.clone()));
    }
}
