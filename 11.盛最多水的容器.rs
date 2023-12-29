/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let length = height.len();
        let (mut i, mut j) = (0, length - 1);
        let mut cap = 0;
        while i < j {
            let area = (height[i].min(height[j])) * (j - i) as i32;
            cap = cap.max(area);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        cap
    }
}
// @lc code=end
