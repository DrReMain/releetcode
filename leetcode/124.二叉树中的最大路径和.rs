/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::find_max_path_sum(&mut max_sum, &root);
        max_sum
    }
    pub fn find_max_path_sum(sum: &mut i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_max = 0.max(Self::find_max_path_sum(sum, &node.borrow().left));
            let right_max = 0.max(Self::find_max_path_sum(sum, &node.borrow().right));
            *sum = (*sum).max(node.borrow().val + left_max + right_max);
            node.borrow().val + left_max.max(right_max)
        } else {
            0
        }
    }
}
// @lc code=end

