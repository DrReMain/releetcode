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
    let [x, y] = [m - 1, n - 1];
    for (let i = m + n - 1; i >= 0; --i) {
        let n1 = x >= 0 ? nums1[x] : -Infinity;
        let n2 = y >= 0 ? nums2[y] : -Infinity;

        if (n1 > n2) {
            nums1[i] = n1;
            x--;
        } else {
            nums1[i] = n2;
            y--;
        }
    }
};
// @lc code=end

