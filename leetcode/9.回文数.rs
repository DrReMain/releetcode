/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut origin = x;
        let mut ret = 0;
        while origin > 0 {
            ret = ret * 10 + origin % 10;
            origin = origin / 10;
        }
        return ret == x;
    }
}
// @lc code=end
