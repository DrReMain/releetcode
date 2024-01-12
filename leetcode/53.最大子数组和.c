/*
 * @lc app=leetcode.cn id=53 lang=c
 *
 * [53] 最大子数组和
 */

// @lc code=start
int max(int a, int b) {
    if (a > b) return a;
    return b;
}
int maxSubArray(int* nums, int numsSize) {
    int pre = 0;
    int ret = nums[0];
    for (int i = 0; i < numsSize; ++i) {
        pre = max(pre + nums[i], nums[i]);
        ret = max(ret, pre);
    }
    return ret;
}
// @lc code=end

