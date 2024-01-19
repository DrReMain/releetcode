/*
 * @lc app=leetcode.cn id=695 lang=typescript
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
function maxAreaOfIsland(grid: number[][]): number {
    let ret = 0;
    const [m, n] = [grid.length, grid[0].length];
    for (let i = 0; i < m; ++i) {
        for (let j = 0; j < n; ++j) {
            if (grid[i][j] === 1) ret = Math.max(ret, dfs(grid, i, j));
        }
    }
    return ret;
};
function dfs(grid: number[][], x: number, y: number): number {
    const [m, n] = [grid.length, grid[0].length];
    let area = 0;
    if (x < 0 || y < 0 || x >= m || y >= n || grid[x][y] === 0) return area;
    grid[x][y] = 0;
    area++;
    area += dfs(grid, x + 1, y);
    area += dfs(grid, x - 1, y);
    area += dfs(grid, x, y + 1);
    area += dfs(grid, x, y - 1);
    return area;
}
// @lc code=end

