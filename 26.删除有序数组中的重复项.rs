/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let mut m: HashSet<i32> = HashSet::new();
        let mut cur = 0usize;

        for i in 0..nums.len() {
            let n = nums[i];
            if m.insert(n) {
                nums[cur] = n;
                cur += 1;
            }
        }

        cur as i32
    }
}
// @lc code=end
