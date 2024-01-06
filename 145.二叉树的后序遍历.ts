/*
 * @lc app=leetcode.cn id=145 lang=typescript
 *
 * [145] 二叉树的后序遍历
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     val: number
 *     left: TreeNode | null
 *     right: TreeNode | null
 *     constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.left = (left===undefined ? null : left)
 *         this.right = (right===undefined ? null : right)
 *     }
 * }
 */

function postorderTraversal(root: TreeNode | null): number[] {
    // let ret: number[] = [];
    // if (root) {
    //     ret = ret.concat(postorderTraversal(root.left));
    //     ret = ret.concat(postorderTraversal(root.right));
    //     ret.push(root.val);
    // }
    // return ret; 

    const ret: number[] = [];
    const stack: TreeNode[] = [];
    let last: TreeNode | null = null;
    while (root || stack.length > 0) {
        while (root) {
            stack.push(root);
            root = root.left;
        }

        const tail = stack[stack.length - 1];
        if (tail.right && tail.right !== last) {
            root = tail.right;
        } else {
            ret.push(tail.val);
            last = stack.pop();
        }
    }
    return ret;
};
// @lc code=end

