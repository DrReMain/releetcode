/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret = Vec::<Vec<i32>>::new();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    ret.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left+1] == nums[left] {
                        left += 1;
                    }
                    while left < right && nums[right-1] == nums[right] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
            }
        }

        ret
    }
}
// @lc code=end
