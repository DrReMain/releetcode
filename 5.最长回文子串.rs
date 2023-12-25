/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let arr: Vec<char> = s.chars().collect();
        let (mut start_idx, mut max) = (0, 1);
        for i in 0..arr.len() {
            let (start1, len1) = Solution::expand_center_around(&arr, i, i);
            let (start2, len2) = Solution::expand_center_around(&arr, i, i + 1);
            if len2 > len1 {
                if len2 > max {
                    start_idx = start2;
                    max = len2;
                }
            } else {
                if len1 > max {
                    start_idx = start1;
                    max = len1;
                }
            }
        }
        arr[start_idx..start_idx+max].iter().collect(
    }
    fn expand_center_around(s: &Vec<char>, mut lt: usize, mut rt: usize) -> (usize, usize) {
        while rt < s.len() && s[lt] == s[rt] {
            if lt == 0 {
                return (0, rt + 1);
            }
            lt -= 1;
            rt += 1;
        }
        (lt + 1, rt - lt - 1)
    }
}
// @lc code=end
