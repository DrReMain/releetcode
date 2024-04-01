/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 */

// @lc code=start
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());

        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    ret = ret.max(Self::max_area_of_island_dfs(&mut grid, i, j));
                }
            }
        }
        ret
    }
    pub fn max_area_of_island_dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        if x >= m || y >= n || grid[x][y] == 0 {
            return 0;
        }

        grid[x][y] = 0;

        1 + Self::max_area_of_island_dfs(grid, x + 1, y)
            + if let Some(n) = x.checked_sub(1) {
                Self::max_area_of_island_dfs(grid, n, y)
            } else {
                0
            }
            + Self::max_area_of_island_dfs(grid, x, y + 1)
            + if let Some(n) = y.checked_sub(1) {
                Self::max_area_of_island_dfs(grid, x, n)
            } else {
                0
            }
    }
}
// @lc code=end
