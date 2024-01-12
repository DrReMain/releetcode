/*
 * @lc app=leetcode.cn id=88 lang=typescript
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
/**
 Do not return anything, modify nums1 in-place instead.
 */
function merge(nums1: number[], m: number, nums2: number[], n: number): void {
    let [p, x, y] = [nums1.length - 1, m - 1, n - 1];
    while (x >= 0 && y >= 0) {
        nums1[p--] = nums1[x] < nums2[y] ? nums2[y--] : nums1[x--];
    }
    for (let i = 0; i <= y; i++) {
        nums1[i] = nums2[i];
    }
};
// @lc code=end

