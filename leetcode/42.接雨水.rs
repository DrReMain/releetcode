/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut leftMax, mut rightMax) = (0, 0);
        let mut ret = 0;
        while left < right {
            leftMax = leftMax.max(height[left]);
            rightMax = rightMax.max(height[right]);
            if height[left] < height[right] {
                ret += leftMax - height[left];
                left += 1;
            } else {
                ret += rightMax - height[right];
                right -= 1;
            }
        }
        ret
    }
}
// @lc code=end

