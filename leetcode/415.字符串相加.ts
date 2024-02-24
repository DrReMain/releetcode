/*
 * @lc app=leetcode.cn id=415 lang=typescript
 *
 * [415] 字符串相加
 */

// @lc code=start
function addStrings(num1: string, num2: string): string {
    let [i1, i2, carry] = [num1.length-1, num2.length-1, 0];
    let ret = "";
    while (i1 >= 0 || i2 >= 0 || carry > 0) {
        let [n1, n2] = [0, 0];
        if (i1 >= 0) n1 = num1.charCodeAt(i1) - 48;
        if (i2 >= 0) n2 = num2.charCodeAt(i2) - 48;

        const sum = n1 + n2 + carry;
        ret = sum%10 + ret;
        carry = Math.trunc(sum/10);
        --i1;
        --i2;
    }
    return ret;
};
// @lc code=end

