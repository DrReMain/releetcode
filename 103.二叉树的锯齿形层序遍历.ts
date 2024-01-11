/*
 * @lc app=leetcode.cn id=103 lang=typescript
 *
 * [103] 二叉树的锯齿形层序遍历
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

function zigzagLevelOrder(root: TreeNode | null): number[][] {
    const ret: number[][] = [];
    if (!root) return ret;
    let queue = [root];
    let flag = true;

    while (queue.length > 0) {
        const len = queue.length;
        const level: number[] = [];
        for (let i = 0; i < len; i++) {
            const node = flag ? queue[i] : queue[len - 1 - i];
            level.push(node.val);
            if (node.left) queue.push(node.left);
            if (node.right) queue.push(node.right);
        }
        ret.push(level);
        queue = queue.slice(len);
        
        flag = !flag;
    }
    return ret;
};
// @lc code=end

