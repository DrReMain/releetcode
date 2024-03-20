use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(n) => {
                let n = n.borrow();
                Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone())) + 1
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_level_order1() {
        let root = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let expected = 3;
        assert_eq!(Solution::max_depth(root), expected);
    }

    #[test]
    fn test_level_order2() {
        let root = TreeNode::from(vec![Some(1), None, Some(2)]);
        let expected = 2;
        assert_eq!(Solution::max_depth(root), expected);
    }

    #[bench]
    fn bench_level_order(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        b.iter(|| Solution::max_depth(root.clone()));
    }
}
