/*
 * @lc app=leetcode.cn id=32 lang=typescript
 *
 * [32] 最长有效括号
 */

// @lc code=start
function longestValidParentheses(s: string): number {
    let [left, right, ret] = [0, 0, 0];
    for (let i = 0; i < s.length; ++i) {
        if (s[i] === '(') left++;
        else right++;
        if (left === right) ret = Math.max(ret, right*2);
        else if (right > left) [left, right] = [0, 0];
    }
    [left, right] = [0, 0];
    for (let i = s.length - 1; i >= 0; i--) {
        if (s[i] === ')') right++;
        else left++;
        if (left === right) ret = Math.max(ret, left*2);
        else if (left > right) [left, right] = [0, 0];
    }
    return ret;
};
// @lc code=end

