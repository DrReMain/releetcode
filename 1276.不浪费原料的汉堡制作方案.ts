/*
 * @lc app=leetcode.cn id=1276 lang=typescript
 *
 * [1276] 不浪费原料的汉堡制作方案
 */

// @lc code=start
function numOfBurgers(tomatoSlices: number, cheeseSlices: number): number[] {
    if (tomatoSlices % 2 !== 0 ||
        tomatoSlices < cheeseSlices * 2 ||
        cheeseSlices * 4 < tomatoSlices) return []

    return [(tomatoSlices >> 1) - cheeseSlices, cheeseSlices * 2 - (tomatoSlices >> 1)]
};
// @lc code=end

