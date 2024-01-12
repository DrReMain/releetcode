/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut nums = nums;
        nums.sort_by(|a, b| a.cmp(b));
        let mut result = HashSet::new();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    result.insert(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left + 1] == nums[left] {
                        left += 1;
                    }
                    while left < right && nums[right - 1] == nums[right] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }
        result.into_iter().collect()
    }
}
// @lc code=end
