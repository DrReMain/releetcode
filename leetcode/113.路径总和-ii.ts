/*
 * @lc app=leetcode.cn id=113 lang=typescript
 *
 * [113] 路径总和 II
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

function pathSum(root: TreeNode | null, targetSum: number): number[][] {
    const ret: number[][] = [];
    if (!root) return ret;

    function dfs(node: TreeNode| null, path: number[], sum: number) {
        if (!node) return;

        sum += node.val;
        path.push(node.val);

        if (!node.left && !node.right && sum === targetSum) {
            ret.push([...path]);
            return;
        }
        dfs(node.left, [...path], sum);
        dfs(node.right, [...path], sum);
    }
    dfs(root, [], 0);
    return ret;
};
// @lc code=end

