/*
 * @lc app=leetcode.cn id=26 lang=typescript
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
function removeDuplicates(nums: number[]): number {
    const m = new Map<number, boolean>();
    let cur = -1;
    for (let i = 0; i < nums.length; ++i) {
        if (!m.get(nums[i])) {
            nums[++cur] = nums[i];
            m.set(nums[i], true);
        }
    }
    return cur + 1;
};
// @lc code=end

