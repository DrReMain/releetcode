/*
 * @lc app=leetcode.cn id=3 lang=typescript
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
function lengthOfLongestSubstring(s: string): number {
    let l = s.length;
    if (l < 2) return l;

    const m = new Map<string, number>();
    let ret = 0;
    for (let cur = 0, i = 0; i < l; ++i) {
        if (m.has(s[i]))
            cur = Math.max(cur, m.get(s[i])! + 1);
        ret = Math.max(ret, i - cur + 1);
        m.set(s[i], i);
    }

    return ret;
};
// @lc code=end

