/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        nums.iter().fold(nums[0], |ret, &n| {
            prev = n.max(prev + n);
            ret.max(prev)
        })
    }
}
// @lc code=end
