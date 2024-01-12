/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut right = 0usize;
        let mut i = 0usize;
        while i < nums.len() {
            if nums[i] != val {
                right += 1;
                nums[right - 1] = nums[i];
            }
            i += 1;
        }
        right as i32
    }
}
// @lc code=end
