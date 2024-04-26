/*
 * @lc app=leetcode.cn id=572 lang=rust
 *
 * [572] 另一棵树的子树
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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root, sub_root) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                Self::is_subtree_check(Some(a.clone()), Some(b.clone()))
                    || Self::is_subtree(a.borrow().left.clone(), Some(b.clone()))
                    || Self::is_subtree(a.borrow().right.clone(), Some(b.clone()))
            }
        }
    }
    pub fn is_subtree_check(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(a), Some(b)) => {
                a.borrow().val == b.borrow().val
                    && Self::is_subtree_check(a.borrow().left.clone(), b.borrow().left.clone())
                    && Self::is_subtree_check(a.borrow().right.clone(), b.borrow().right.clone())
            }
        }
    }
}
// @lc code=end
