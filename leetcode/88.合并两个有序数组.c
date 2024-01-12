/*
 * @lc app=leetcode.cn id=88 lang=c
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
void merge(int* nums1, int nums1Size, int m, int* nums2, int nums2Size, int n) {
    int p = nums1Size - 1;
    int x = m - 1;
    int y = n - 1;
    while (x >= 0 && y >= 0) {
        nums1[p--] = nums1[x] < nums2[y] ? nums2[y--] : nums1[x--];
    }
    for (int i = 0; i <= y; i++) {
        nums1[i] = nums2[i];
    }
}
// @lc code=end

