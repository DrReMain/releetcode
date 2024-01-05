/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ret = 0;
        let (mut left, mut right) = (0i32, height.len() as i32 - 1);
        while left < right {
            let area = height[left as usize].min(height[right as usize]) * (right-left);
            ret = ret.max(area);
            if height[left as usize] < height[right as usize] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ret
    }
}
// @lc code=end
