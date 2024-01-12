/*
 * @lc app=leetcode.cn id=236 lang=rust
 *
 * [236] 二叉树的最近公共祖先
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if Rc::ptr_eq(&node, p.as_ref().unwrap()) || Rc::ptr_eq(&node, q.as_ref().unwrap()) {
                return Some(node);
            }

            let left =
                Self::lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right =
                Self::lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

            match (left.clone(), right.clone()) {
                (Some(_), Some(_)) => Some(node),
                (Some(_), None) => left,
                (None, Some(_)) => right,
                (None, None) => None,
            }
        } else {
            None
        }
    }
}
// @lc code=end

