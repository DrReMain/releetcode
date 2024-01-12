/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p, mut x, mut y) = (nums1.len() - 1, m as usize, n as usize);
        while x > 0 && y > 0 {
            if nums1[x - 1] < nums2[y - 1] {
                nums1[p] = nums2[y - 1];
                y -= 1;
            } else {
                nums1[p] = nums1[x - 1];
                x -= 1;
            }
            p -= 1;
        }
        for i in 0..y {
            nums1[i] = nums2[i];
        }
    }
}
// @lc code=end

