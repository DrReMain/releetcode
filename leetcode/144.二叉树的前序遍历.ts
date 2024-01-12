/*
 * @lc app=leetcode.cn id=144 lang=typescript
 *
 * [144] 二叉树的前序遍历
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

function preorderTraversal(root: TreeNode | null): number[] {
    let ret: number[] = [];
    if (root) {
        ret.push(root.val);
        ret = ret.concat(preorderTraversal(root.left));
        ret = ret.concat(preorderTraversal(root.right));
    }
    return ret;

    // const ret: number[] = [];
    // const stack: TreeNode[] = [];
    // if (root) stack.push(root);
    // while (stack.length > 0) {
    //     ret.push(stack.pop()!.val);
    //     if (root.right) stack.push(root.right);
    //     if (root.left) stack.push(root.left);
    // }
    // return ret;
};
// @lc code=end

