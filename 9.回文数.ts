/*
 * @lc app=leetcode.cn id=9 lang=typescript
 *
 * [9] 回文数
 */

// @lc code=start
function isPalindrome(x: number): boolean {
    if (x < 0) return false;
    let origin = x;
    let ret = 0;
    while (origin > 0) {
        ret = ret * 10 + origin % 10;
        origin = Math.floor(origin / 10);
    }
    return ret === x;
};
// @lc code=end

