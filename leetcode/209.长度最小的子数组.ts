/*
 * @lc app=leetcode.cn id=209 lang=typescript
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
function minSubArrayLen(target: number, nums: number[]): number {
    const length = nums.length;
    if (length === 0) return 0;

    let ret = +Infinity;
    let [start, end] = [0, 0];
    let sum = 0;
    while (end < length) {
        sum += nums[end];
        while (sum >= target) {
            ret = Math.min(ret, end-start+1);
            sum -= nums[start];
            start++;
        }
        end++;
    }
    return ret === +Infinity ? 0 : ret;
};
// @lc code=end

