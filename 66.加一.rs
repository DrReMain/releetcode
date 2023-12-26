/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;

        for digit in digits.iter_mut().rev() {
            *digit += carry;
            carry = *digit / 10;
            *digit %= 10;
        }

        if carry > 0 {
            let mut result = vec![carry];
            result.extend(digits);
            result
        } else {
            digits
        }
    }
}
// @lc code=end
