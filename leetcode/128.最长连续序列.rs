/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums.into_iter().collect::<HashSet<_>>();
        let mut ret = 0;
        for &n in set.iter() {
            if !set.contains(&(n - 1)) {
                let mut p = n;
                let mut max = 1;
                while set.contains(&(p + 1)) {
                    p += 1;
                    max += 1;
                }
                ret = ret.max(max);
            }
        }
        ret
    }
}
// @lc code=end
