/*
 * @lc app=leetcode.cn id=162 lang=typescript
 *
 * [162] 寻找峰值
 */

// @lc code=start
function findPeakElement(nums: number[]): number {
    return nums.reduce<number>((ret, next, idx) => next > nums[ret] ? idx : ret, 0);
};
// @lc code=end

