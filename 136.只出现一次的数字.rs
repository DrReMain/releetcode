/*
 * @lc app=leetcode.cn id=136 lang=rust
 *
 * [136] 只出现一次的数字
 */

// @lc code=start
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ret = nums[0];
        for i in 1..nums.len() {
            ret ^= nums[i];
        }
        ret
    }
}
// @lc code=end

