/*
 * @lc app=leetcode.cn id=162 lang=typescript
 *
 * [162] 寻找峰值
 */

// @lc code=start
function findPeakElement(nums: number[]): number {
    let idx = 0;
    for (let i = 0; i < nums.length; ++i)
        if (nums[i] > nums[idx]) idx = i;
    return idx;
};
// @lc code=end

