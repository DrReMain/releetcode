/*
 * @lc app=leetcode.cn id=338 lang=typescript
 *
 * [338] 比特位计数
 */

// @lc code=start
function countBits(n: number): number[] {
    return Array(n+1).fill(null).map((_, idx) => {
        let count = 0;
        while (idx > 0) {
            if (idx%2 !== 0) count++;
            idx >>= 1;
        }
        return count;
    });
};
// @lc code=end

