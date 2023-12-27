/*
 * @lc app=leetcode.cn id=2678 lang=typescript
 *
 * [2678] 老人的数目
 */

// @lc code=start
function countSeniors(details: string[]): number {
    let ret = 0;
    for (let p of details) {
        if (p[11] > '6') ret++;
        if (p[11] === '6' && p[12] > '0') ret++;
    }
    return ret;
};
// @lc code=end

