/*
 * @lc app=leetcode.cn id=124 lang=typescript
 *
 * [124] 二叉树中的最大路径和
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

function maxPathSum(root: TreeNode | null): number {
    let maxSum = Number.MIN_SAFE_INTEGER;
    function findMaxPathSum(node: TreeNode | null) {
        if (!node) return 0;
        let leftMax = Math.max(findMaxPathSum(node.left), 0);
        let rightMax = Math.max(findMaxPathSum(node.right), 0);
        maxSum = Math.max(maxSum, node.val + leftMax + rightMax);
        return node.val + Math.max(leftMax, rightMax);
    }
    findMaxPathSum(root);
    return maxSum; 
};
// @lc code=end

