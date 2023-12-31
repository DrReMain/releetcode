/*
 * @lc app=leetcode.cn id=415 lang=rust
 *
 * [415] 字符串相加
 */

// @lc code=start
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let (num1, num2) = (
            num1.chars().collect::<Vec<char>>(),
            num2.chars().collect::<Vec<char>>(),
        );
        let mut ret = String::new();
        let (mut carry, mut i, mut j) = (0, num1.len() as i32 - 1, num2.len() as i32 - 1);

        while i >= 0 || j >= 0 || carry > 0 {
            let digit1 = if i >= 0 {
                num1[i as usize] as u8 - b'0'
            } else {
                0
            };
            let digit2 = if j >= 0 {
                num2[j as usize] as u8 - b'0'
            } else {
                0
            };

            let sum = digit1 + digit2 + carry;
            carry = sum / 10;

            ret.insert(0, (sum % 10 + b'0') as char);

            i -= 1;
            j -= 1;
        }
        ret
    }
}
// @lc code=end

