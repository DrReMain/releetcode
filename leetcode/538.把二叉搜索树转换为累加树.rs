/*
 * @lc app=leetcode.cn id=538 lang=rust
 *
 * [538] 把二叉搜索树转换为累加树
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
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;
        let mut root = root;
        Self::convert_bst_dfs(&mut sum, &mut root);
        root
    }
    pub fn convert_bst_dfs(sum: &mut i32, root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            Self::convert_bst_dfs(sum, &mut node.borrow_mut().right);
            *sum += node.borrow().val;
            node.borrow_mut().val = *sum;
            Self::convert_bst_dfs(sum, &mut node.borrow_mut().left);
        }
    }
}
// @lc code=end

