/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut pre, mut ret) = (0, nums[0]);
        nums.iter().for_each(|&x| {
            pre = x.max(pre + x);
            ret = ret.max(pre);
        });
        ret
    }
}
// @lc code=end

