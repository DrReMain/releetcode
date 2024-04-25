/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut ret = 0;
        while left < right {
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);
            
            if height[left] < height[right] {
                ret += left_max - height[left];
                left += 1;
            } else {
                ret += right_max - height[right];
                right -= 1;
            }
        }
        ret
    }
}
// @lc code=end

