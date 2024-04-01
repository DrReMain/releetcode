/*
 * @lc app=leetcode.cn id=695 lang=typescript
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
function maxAreaOfIsland(grid: number[][]): number {
    const [m, n] = [grid.length, grid[0].length];

    let ret = 0;
    for (let i = 0; i < m; ++i)
        for (let j = 0; j < n; ++j)
            if (grid[i][j] === 1)
                ret = Math.max(ret, dfs(grid, i, j));
    return ret;
};
function dfs(grid: number[][], x: number, y: number): number {
    const [m, n] = [grid.length, grid[0].length];
    if (x < 0 || y < 0 || x >= m || y >= n || grid[x][y] === 0) return 0;

    grid[x][y] = 0;
    return 1 + dfs(grid, x + 1, y) + dfs(grid, x - 1, y) + dfs(grid, x, y + 1) + dfs(grid, x, y - 1);
}
// @lc code=end

