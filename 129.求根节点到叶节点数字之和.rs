/*
 * @lc app=leetcode.cn id=129 lang=rust
 *
 * [129] 求根节点到叶节点数字之和
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::sum_numbers_dfs(root, 0);
    }
    pub fn sum_numbers_dfs(root: Option<Rc<RefCell<TreeNode>>>, prev_sum: i32) -> i32 {
        match root {
            Some(node) => {
                let node = node.borrow();
                let sum = prev_sum * 10 + node.val;
                if let (None, None) = (&node.left, &node.right) {
                    sum
                } else {
                    Solution::sum_numbers_dfs(node.left.clone(), sum)
                        + Solution::sum_numbers_dfs(node.right.clone(), sum)
                }
            }
            None => 0,
        }
    }
}
// @lc code=end
