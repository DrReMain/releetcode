use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum_numbers_dfs(root, 0)
    }
    pub fn sum_numbers_dfs(root: Option<Rc<RefCell<TreeNode>>>, prev_sum: i32) -> i32 {
        if let Some(node) = root {
            let bo_node = node.borrow();
            let sum = prev_sum * 10 + bo_node.val;

            if bo_node.left.is_none() && bo_node.right.is_none() {
                sum
            } else {
                Self::sum_numbers_dfs(bo_node.left.clone(), sum)
                    + Self::sum_numbers_dfs(bo_node.right.clone(), sum)
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_sum_numbers1() {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        assert_eq!(Solution::sum_numbers(root), 25);
    }

    #[test]
    fn test_sum_numbers2() {
        let root = TreeNode::from(vec![Some(4), Some(9), Some(0), Some(5), Some(1)]);
        assert_eq!(Solution::sum_numbers(root), 1026);
    }

    #[test]
    fn test_sum_numbers3() {
        let root = TreeNode::from(vec![Some(1), Some(0)]);
        assert_eq!(Solution::sum_numbers(root), 10);
    }

    #[bench]
    fn bench_sum_numbers(b: &mut Bencher) {
        let root = TreeNode::from(vec![Some(1), Some(2), Some(3)]);
        b.iter(|| Solution::sum_numbers(root.clone()));
    }
}
