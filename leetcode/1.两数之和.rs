/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, v) in nums.iter().enumerate() {
            if let Some(n) = map.get(&(target - v)) {
                return vec![*n, idx as i32];
            } else {
                map.insert(*v, idx as i32);
            }
        }

        panic!()
    }
}
// @lc code=end
