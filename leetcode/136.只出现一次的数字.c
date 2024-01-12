/*
 * @lc app=leetcode.cn id=136 lang=c
 *
 * [136] 只出现一次的数字
 */

// @lc code=start
int singleNumber(int* nums, int numsSize) {
    int ret = nums[0];
    for (int i = 1; i < numsSize; ++i) {
        ret ^= nums[i];
    }
    return ret;
}
// @lc code=end

