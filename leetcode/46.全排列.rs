/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 */

// @lc code=start
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        fn backtrack(path: &mut Vec<i32>, nums: &Vec<i32>, ret: &mut Vec<Vec<i32>>) {
            if path.len() == nums.len() {
                ret.push(path.clone());
                return;
            }

            for i in 0..nums.len() {
                if path.contains(&nums[i]) {
                    continue;
                }
                path.push(nums[i]);
                backtrack(path, nums, ret);
                path.pop();
            }
        }

        backtrack(Vec::new().as_mut(), &nums, &mut ret);
        ret
    }
}
// @lc code=end

