/*
 * @lc app=leetcode.cn id=14 lang=typescript
 *
 * [14] 最长公共前缀
 */

// @lc code=start
function longestCommonPrefix(strs: string[]): string {
    if (strs.length === 0) return "";
    strs.sort();
    let [start, end] = [strs[0], strs[strs.length-1]];
    let i = 0;
    while (i < start.length && i < end.length && start[i] === end[i])
        i++;
    return start.substring(0, i);
};
// @lc code=end

