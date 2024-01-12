/*
 * @lc app=leetcode.cn id=58 lang=typescript
 *
 * [58] 最后一个单词的长度
 */

// @lc code=start
function lengthOfLastWord(s: string): number {
    let [left, right] = [-1, -1];
    for (let i = s.length - 1; i >= 0; i--) {
        if (right === -1 && s[i] !== ' ') right = i;
        if (right !== -1 && s[i] === ' ') {
            left = i;
            break;
        }
    }
    return right - left;
};
// @lc code=end

