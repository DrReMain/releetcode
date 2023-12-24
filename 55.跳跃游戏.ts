/*
 * @lc app=leetcode.cn id=55 lang=typescript
 *
 * [55] 跳跃游戏
 */

// @lc code=start
function canJump(nums: number[]): boolean {
    const length = nums.length;
    let most = 0;
    for (let i = 0; i < length; ++i) {
        if (i <= most) {
            most = Math.max(most, i + nums[i]);
            if (most >= length - 1) {
                return true;
            }
        }
    }
    return false;
};
// @lc code=end

