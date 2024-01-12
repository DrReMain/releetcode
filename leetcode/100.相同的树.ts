/*
 * @lc app=leetcode.cn id=100 lang=typescript
 *
 * [100] 相同的树
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

function isSameTree(p: TreeNode | null, q: TreeNode | null): boolean {
    return dfs(p, q);
};
function dfs(x: TreeNode | null, y: TreeNode | null): boolean {
    if (x === null && y === null) return true;
    if (x === null || y === null) return false;
    return x.val === y.val && dfs(x.left, y.left) && dfs(x.right, y.right);
}
// @lc code=end

