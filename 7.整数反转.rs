/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut ret = 0;

        while x != 0 {
            if ret > i32::MAX / 10 || ret < i32::MIN / 10 {
                return 0;
            }
            ret = ret * 10 + x % 10;
            x /=10;
        }

        ret
    }
}
// @lc code=end
