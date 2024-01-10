/*
 * @lc app=leetcode.cn id=4 lang=typescript
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
function findMedianSortedArrays(nums1: number[], nums2: number[]): number {
    const total = nums1.length + nums2.length;
    let [i, j] = [0, 0];
    let [r1, r2] = [0, 0];
    while (i + j < (total >> 1) + 1) {
        r1 = r2;

        if (i < nums1.length && j < nums2.length) {
            if (nums1[i] > nums2[j]) {
                r2 = nums2[j];
                j++;
            } else {
                r2 = nums1[i];
                i++;
            }
        } else if (i < nums1.length) {
            r2 = nums1[i];
            i++;
        } else if (j < nums2.length) {
            r2 = nums2[j];
            j++;
        }
    }
    if (total % 2 === 0) return (r1 + r2) / 2;
    return r2;
};
// @lc code=end

