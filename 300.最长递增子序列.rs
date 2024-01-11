/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut tmp: Vec<i32> = Vec::new();
        for n in nums {
            let length = tmp.len();
            if length == 0 || n > tmp[length - 1] {
                tmp.push(n);
                continue;
            }

            let (mut left, mut right) = (0, length as i32 - 1);
            let mut p = right;
            while left <= right {
                let mid = (left + right) >> 1;
                if tmp[mid as usize] >= n {
                    p = mid;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            tmp[p as usize] = n;
        }
        tmp.len() as i32
    }
}
// @lc code=end

