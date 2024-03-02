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

    let q = [root];
    let isOddLevel = true;

    while (q.length) {
        const level: number[] = [];
        const newQ: TreeNode[] = [];

        if (isOddLevel)
            for (let i = 0; i < q.length; ++i) {
                const node = q[i];
                level.push(node.val);
                if (node.left) newQ.push(node.left);
                if (node.right) newQ.push(node.right);
            }
        else
            for (let i = q.length-1; i >= 0; --i) {
                const node = q[i];
                level.push(node.val);
                if (node.right) newQ.unshift(node.right);
                if (node.left) newQ.unshift(node.left);
            }

        ret.push(level);
        q = newQ;
        isOddLevel = !isOddLevel;
    }
    return ret;
};
// @lc code=end

