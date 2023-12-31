/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let node_borrow = node.borrow();
            if node_borrow.left.is_none() && node_borrow.right.is_none() {
                return target_sum == node_borrow.val;
            }

            return Solution::has_path_sum(node_borrow.left.clone(), target_sum-node_borrow.val)
                || Solution::has_path_sum(node_borrow.right.clone(), target_sum-node_borrow.val);
        }
        false
    }
}
// @lc code=end

