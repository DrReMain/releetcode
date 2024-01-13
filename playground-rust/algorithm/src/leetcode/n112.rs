use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let bo_node = node.borrow();

            if bo_node.left.is_none() && bo_node.right.is_none() {
                target_sum == bo_node.val
            } else {
                Self::has_path_sum(bo_node.left.clone(), target_sum - bo_node.val)
                    || Self::has_path_sum(bo_node.right.clone(), target_sum - bo_node.val)
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_has_path_sum1() {
        let root = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        assert_eq!(Solution::has_path_sum(root, 22), true);
    }

    #[test]
    fn test_has_path_sum2() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::has_path_sum(root, 5), false);
    }

    #[test]
    fn test_has_path_sum3() {
        let root = TreeNode::from(vec![]);
        assert_eq!(Solution::has_path_sum(root, 0), false);
    }

    #[bench]
    fn bench_has_path_sum(b: &mut Bencher) {
        let root = TreeNode::from(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        b.iter(|| Solution::has_path_sum(root.clone(), 22));
    }
}
