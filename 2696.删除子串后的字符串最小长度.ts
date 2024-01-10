/*
 * @lc app=leetcode.cn id=2696 lang=typescript
 *
 * [2696] 删除子串后的字符串最小长度
 */

// @lc code=start
function minLength(s: string): number {
    let stack: string[] = [];
    for (let i = 0; i < s.length; ++i) {
        stack.push(s[i]);
        const len = stack.length;
        if (len >= 0 && (
            stack.slice(len - 2).join("") === "AB" ||
            stack.slice(len - 2).join("") === "CD"
        )) stack = stack.slice(0, len - 2);
    }
    return stack.length;
};
// @lc code=end

