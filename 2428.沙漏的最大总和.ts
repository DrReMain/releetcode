/*
 * @lc app=leetcode.cn id=2428 lang=typescript
 *
 * [2428] 沙漏的最大总和
 */

// @lc code=start
function maxSum(grid: number[][]): number {
    let [m, n] = [grid.length, grid[0].length];
    let ret = 0;
    for (let i = 1; i < m - 1; ++i) {
        for (let j = 1; j < n - 1; ++j) {
            let tmp = grid[i][j] +
                    grid[i-1][j] +
                    grid[i+1][j] +
                    grid[i-1][j-1] +
                    grid[i+1][j+1] +
                    grid[i-1][j+1] +
                    grid[i+1][j-1];
            ret = Math.max(ret, tmp);
        }
    }
    return ret;
};
// @lc code=end

