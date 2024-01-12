/*
 * @lc app=leetcode.cn id=3 lang=typescript
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
    const m = new Map<string, boolean>();
    let ret = 0;
    let cur = 0;
    for (let i = 0; i < s.length; ++i) {
        if (i > 0) m.set(s[i - 1], false);
        while (cur < s.length && !m.get(s[cur])) {
            m.set(s[cur], true);
            cur++;
        }
        ret = Math.max(ret, cur - i);
    }
    return ret;
};
// @lc code=end

