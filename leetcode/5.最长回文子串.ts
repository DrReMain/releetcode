/*
 * @lc app=leetcode.cn id=5 lang=typescript
 *
 * [5] 最长回文子串
 */

// @lc code=start
function longestPalindrome(s: string): string {
    let [start, end] = [0, 0];
    for (let i = 0; i < s.length; ++i) {
        let [left1, right1] = expandCenterAround(s, i, i);
        let [left2, right2] = expandCenterAround(s, i, i + 1);
        if (right1 - left1 > end - start) [start, end] = [left1, right1];
        if (right2 - left2 > end - start) [start, end] = [left2, right2];
    }
    return s.substring(start, end + 1);
};
function expandCenterAround(s: string, left: number, right: number): [number, number] {
    for (; left >= 0 && right < s.length && s[left] === s[right]; left -= 1, right += 1) { }
    return [left + 1, right - 1];
}

// @lc code=end

