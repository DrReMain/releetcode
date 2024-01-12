/*
 * @lc app=leetcode.cn id=344 lang=typescript
 *
 * [344] 反转字符串
 */

// @lc code=start
/**
 Do not return anything, modify s in-place instead.
 */
function reverseString(s: string[]): void {
    for (
        let left = 0, right = s.length - 1;
        left < right;
        left++, right--
    ) {
        s[left] = String.fromCharCode(s[left].charCodeAt(0) ^ s[right].charCodeAt(0))
        s[right] = String.fromCharCode(s[right].charCodeAt(0) ^ s[left].charCodeAt(0))
        s[left] = String.fromCharCode(s[left].charCodeAt(0) ^ s[right].charCodeAt(0))
    }
};
// @lc code=end

