/*
 * @lc app=leetcode.cn id=27 lang=typescript
 *
 * [27] 移除元素
 */

// @lc code=start
function removeElement(nums: number[], val: number): number {
    let [right, i] = [-1, 0];
    while (i < nums.length) {
        if (nums[i] !== val) {
            nums[++right] = nums[i];
        }
        i++;
    }
    return right + 1;
};
// @lc code=end

