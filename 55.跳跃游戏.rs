/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        let mut most = 0usize;
        for (i, n) in nums.iter().enumerate() {
            if i <= most {
                most = most.max(i + (*n) as usize);
                if most >= length - 1 {
                    return true;
                }
            }
        }
        false
    }
}
// @lc code=end
