/*
 * @lc app=leetcode.cn id=6 lang=typescript
 *
 * [6] N 字形变换
 */

// @lc code=start
function convert(s: string, numRows: number): string {
    if (numRows <= 1) return s;

    const ret = Array(numRows).fill("");
    let [dir, flag] = [0, 1];
    for (let i = 0; i < s.length; ++i) {
        ret[dir] += s[i];

        let nextDir = dir + flag;
        if (nextDir >= numRows || nextDir < 0) flag *= -1;
        dir += flag;
    }
    return ret.join("");
};
// @lc code=end

