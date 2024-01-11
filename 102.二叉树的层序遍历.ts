/*
 * @lc app=leetcode.cn id=102 lang=typescript
 *
 * [102] 二叉树的层序遍历
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

function levelOrder(root: TreeNode | null): number[][] {
    const ret: number[][] = [];
    if (!root) return ret;
    let queue = [root];

    while (queue.length > 0) {
        const len = queue.length;
        const level: number[] = [];
        for (let i = 0; i < len; ++i) {
            level.push(queue[i].val);
            if (queue[i].left) queue.push(queue[i].left);
            if (queue[i].right) queue.push(queue[i].right);
        }
        ret.push(level);
        queue = queue.slice(len);
    }
    return ret;
};
// @lc code=end

