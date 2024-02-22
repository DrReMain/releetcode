/*
 * @lc app=leetcode.cn id=1 lang=typescript
 *
 * [1] 两数之和
 */

// @lc code=start
function twoSum(nums: number[], target: number): number[] {
    const dict = new Map<number, number>();
    for (let i = 0; i < nums.length; ++i) {
         const remainIdx = dict.get(nums[i]);
         if (typeof remainIdx === 'number')
             return [remainIdx, i];
         else
             dict.set(target-nums[i], i);
    }
 
    throw new Error();
};
// @lc code=end

