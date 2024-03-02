/*
 * @lc app=leetcode.cn id=31 lang=typescript
 *
 * [31] 下一个排列
 */

// @lc code=start
/**
 Do not return anything, modify nums in-place instead.
 */
function nextPermutation(nums: number[]): void {
    let i = nums.length - 2;
    for (; i >= 0 && nums[i] >= nums[i+1]; --i) {}

    if (i >= 0) {
        let j = nums.length - 1;
        for (; j >= 0 && nums[j] <= nums[i]; --j) {}
        [nums[i], nums[j]] = [nums[j], nums[i]];
    }

    let [left, right] = [i+1, nums.length - 1];
    for (;left < right; left++, right--)
        [nums[left], nums[right]] = [nums[right], nums[left]];
};
// @lc code=end

