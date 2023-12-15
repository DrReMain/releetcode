/*
 * @lc app=leetcode.cn id=1 lang=typescript
 *
 * [1] 两数之和
 */

// @lc code=start
function twoSum(nums: number[], target: number): number[] {
    const m: Map<number, number> = new Map();

    for (let i = 0; i < nums.length; i++) {
        const diff = target - nums[i];

        if (m.has(diff)) {
            return [m.get(diff)!, i];
        }
        m.set(nums[i], i);
    }

    throw new Error();
};
// @lc code=end

