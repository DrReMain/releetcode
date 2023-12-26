/*
 * @lc app=leetcode.cn id=66 lang=typescript
 *
 * [66] 加一
 */

// @lc code=start
function plusOne(digits: number[]): number[] {
    let carry = 1;
    for (let i = digits.length - 1; i >= 0; --i) {
        let sum = digits[i] + carry;
        digits[i] = sum % 10;
        carry = Math.floor(sum / 10);
    }
    if (carry > 0) {
        return [carry].concat(digits);
    } else {
        return digits;
    }
};
// @lc code=end

