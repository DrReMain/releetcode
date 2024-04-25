/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p, mut x, mut y) = (nums1.len() - 1, m as usize, n as usize);
        while x.checked_sub(1).is_some() && y.checked_sub(1).is_some() {
            nums1[p] = if nums1[x - 1] > nums2[y - 1] {
                x -= 1;
                nums1[x]
            } else {
                y -= 1;
                nums2[y]
            };
            p -= 1;
        }

        for i in 0..y {
            nums1[i] = nums2[i];
        }
    }
}
// @lc code=end
