/*
 * @lc app=leetcode.cn id=53 lang=typescript
 *
 * [53] 最大子数组和
 */

// @lc code=start
function maxSubArray(nums: number[]): number {
    let [pre, ret] = [0, nums[0]];
    nums.forEach(x => {
        pre = Math.max(pre+x, x);
        ret = Math.max(ret, pre);
    });
    return ret;
};
// @lc code=end

