/*
 * @lc app=leetcode.cn id=238 lang=typescript
 *
 * [238] 除自身以外数组的乘积
 */

// @lc code=start
function productExceptSelf(nums: number[]): number[] {
    const length = nums.length;
    const ret = Array(length).fill(1);
    for (let i = 1; i < length; ++i)
        ret[i] = nums[i-1]*ret[i-1];

    let p = 1;
    for (let i = length - 1; i >= 0; i--) {
        ret[i] = ret[i] * p;
        p *= nums[i];
    }

    return ret;
};
// @lc code=end

