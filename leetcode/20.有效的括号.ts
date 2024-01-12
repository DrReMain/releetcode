/*
 * @lc app=leetcode.cn id=20 lang=typescript
 *
 * [20] 有效的括号
 */

// @lc code=start
function isValid(s: string): boolean {
    const length = s.length;
    if (length % 2 !== 0) return false;

    const m = new Map([
        [')', '('],
        ['}', '{'],
        [']', '['],
    ])
    const tmp: string[] = [];
    for (let i = 0; i < length; i++) {
        if (tmp.length > 0 && tmp[tmp.length - 1] === m.get(s[i])) {
            tmp.pop();
        } else {
            tmp.push(s[i]);
        }
    }
    return tmp.length === 0;
};
// @lc code=end

