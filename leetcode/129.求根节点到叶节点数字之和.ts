/*
 * @lc app=leetcode.cn id=129 lang=typescript
 *
 * [129] 求根节点到叶节点数字之和
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

function sumNumbers(root: TreeNode | null): number {
    return dfs(root, 0);
};
function dfs(root: TreeNode | null, prevSum: number): number {
    if (root === null) return 0;
    const sum = prevSum * 10 + root.val;
    if (root.left === null && root.right === null) return sum;
    return dfs(root.left, sum) + dfs(root.right, sum);
}
// @lc code=end

