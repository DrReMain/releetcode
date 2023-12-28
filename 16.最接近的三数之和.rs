/*
 * @lc app=leetcode.cn id=16 lang=rust
 *
 * [16] 最接近的三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_by(|a, b| {
            let r = a - b;
            if r > 0 {
                std::cmp::Ordering::Greater
            } else if r < 0 {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        let mut closest = nums[0] + nums[1] + nums[2];
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    return sum;
                }

                if (target - sum).abs() < (target - closest).abs() {
                    closest = sum;
                }
            }
        }
        closest
    }
}
// @lc code=end
