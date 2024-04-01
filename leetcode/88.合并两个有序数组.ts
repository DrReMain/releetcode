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
    let [p, i, j] = [nums1.length-1, m-1, n-1];
    while (i >= 0 && j >= 0)
        nums1[p--] = nums1[i] > nums2[j] ? nums1[i--] : nums2[j--];
    for (let n = 0; n <= j; ++n)
        nums1[n] = nums2[n];
};
// @lc code=end

