/*
 * @lc app=leetcode.cn id=200 lang=typescript
 *
 * [200] 岛屿数量
 */

// @lc code=start
function numIslands(grid: string[][]): number {
    if (grid.length === 0) return 0;
    const [row, col] = [grid.length, grid[0].length];
    let islands = 0;
    for (let i = 0; i < row; ++i) {
        for (let j = 0; j < col; ++j) {
            if (grid[i][j] === '1') {
                islands++;
                dfs(grid, i, j);
            }
        }
    }
    return islands;
};
function dfs(grid: string[][], i: number, j: number) {
    const [row, col] = [grid.length, grid[0].length];
    if (
        i < 0 ||
        j < 0 ||
        i >= row ||
        j >= col ||
        grid[i][j] === '0'
    ) return;

    grid[i][j] = '0';
    dfs(grid, i + 1, j);
    dfs(grid, i - 1, j);
    dfs(grid, i, j + 1);
    dfs(grid, i, j - 1);
}
// @lc code=end

