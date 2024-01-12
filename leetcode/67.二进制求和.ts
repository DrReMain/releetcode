/*
 * @lc app=leetcode.cn id=67 lang=typescript
 *
 * [67] 二进制求和
 */

// @lc code=start
function addBinary(a: string, b: string): string {
    let ret = "";
    let [i, j, carry] = [a.length - 1, b.length - 1, 0];
    while (i >= 0 || j >= 0 || carry > 0) {
        let [n1, n2] = [0, 0];
        if (i >= 0) n1 = a[i].charCodeAt(0) - '0'.charCodeAt(0);
        if (j >= 0) n2 = b[j].charCodeAt(0) - '0'.charCodeAt(0);
        const sum = n1 + n2 + carry;
        ret = sum % 2 + ret;
        carry = Math.floor(sum / 2);

        i--;
        j--;
    }

    return ret;
};
// @lc code=end

