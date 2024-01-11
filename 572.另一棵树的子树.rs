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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(node) = root {
            Self::is_subtree_check(Some(node.clone()), sub_root.clone())
                || Self::is_subtree(node.borrow().left.clone(), sub_root.clone())
                || Self::is_subtree(node.borrow().right.clone(), sub_root.clone())
        } else {
            false
        }
    }
    pub fn is_subtree_check(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(n1), Some(n2)) => {
                let (b1, b2) = (n1.borrow(), n2.borrow());
                if b1.val == b2.val {
                    Self::is_subtree_check(b1.left.clone(), b2.left.clone())
                        && Self::is_subtree_check(b1.right.clone(), b2.right.clone())
                } else {
                    false
                }
            }
        }
    }
}
// @lc code=end

