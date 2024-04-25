/*
 * @lc app=leetcode.cn id=200 lang=rust
 *
 * [200] 岛屿数量
 */

// @lc code=start
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    ret += 1;
                    Self::num_islands_dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        ret
    }
    pub fn num_islands_dfs(grid: &mut Vec<Vec<char>>, x: i32, y: i32) {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        if x < 0 || y < 0 || x >= m || y >= n || grid[x as usize][y as usize] == '0' {
            return;
        }

        grid[x as usize][y as usize] = '0';
        Self::num_islands_dfs(grid, x - 1, y);
        Self::num_islands_dfs(grid, x + 1, y);
        Self::num_islands_dfs(grid, x, y + 1);
        Self::num_islands_dfs(grid, x, y - 1);
    }
}
// @lc code=end
