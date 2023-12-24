/*
 * @lc app=leetcode.cn id=415 lang=typescript
 *
 * [415] 字符串相加
 */

// @lc code=start
function addStrings(num1: string, num2: string): string {
    let [i, j, carry] = [num1.length - 1, num2.length - 1, 0];
    let ret = "";
    
    while (i >= 0 || j >= 0 || carry > 0) {
        let [n1, n2] = [0, 0];
        if (i >= 0) n1 = num1[i].charCodeAt(0) - '0'.charCodeAt(0);
        if (j >= 0) n2 = num2[j].charCodeAt(0) - '0'.charCodeAt(0);

        const sum = n1 + n2 + carry;
        ret = sum%10 + ret;
        carry = Math.floor(sum/10);
        i--;
        j--;
    }

    return ret;
};
// @lc code=end

