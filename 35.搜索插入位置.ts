/*
 * @lc app=leetcode.cn id=35 lang=typescript
 *
 * [35] 搜索插入位置
 */

// @lc code=start
function searchInsert(nums: number[], target: number): number {
    let i = 0
    while (i < nums.length) {
        if (nums[i] >= target) return i;
        ++i;
    }
    return i;
};
// @lc code=end

