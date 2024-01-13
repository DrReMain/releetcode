use leetcode_common::data_structures::binary_tree::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return ret;
        }
        let mut stack = vec![root];
        while stack.len() > 0 {
            let mut level: Vec<i32> = Vec::new();
            let mut new_stack: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            for node in stack {
                if let Some(n) = node {
                    let n = n.borrow();
                    level.push(n.val);
                    if n.left.is_some() {
                        new_stack.push(n.left.clone());
                    }
                    if n.right.is_some() {
                        new_stack.push(n.right.clone());
                    }
                }
            }
            ret.push(level);
            stack = new_stack;
        }
        ret
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
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_level_order2() {
        let root = TreeNode::from(vec![Some(1)]);
        let expected = vec![vec![1]];
        assert_eq!(Solution::level_order(root), expected);
    }

    #[test]
    fn test_level_order3() {
        let root = TreeNode::from(vec![]);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), expected);
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
        b.iter(|| Solution::level_order(root.clone()));
    }
}
