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
                    ret = ret.max(Self::max_area_of_island_dfs(&mut grid, i as i32, j as i32));
                }
            }
        }

        ret
    }
    pub fn max_area_of_island_dfs(grid: &mut Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut area = 0;
        if x < 0 || y < 0 || x >= m || y >= n || grid[x as usize][y as usize] == 0 {
            return area;
        }
        grid[x as usize][y as usize] = 0;
        area += 1;
        area += Self::max_area_of_island_dfs(grid, x + 1, y);
        area += Self::max_area_of_island_dfs(grid, x - 1, y);
        area += Self::max_area_of_island_dfs(grid, x, y + 1);
        area += Self::max_area_of_island_dfs(grid, x, y - 1);
        area
    }
}
// @lc code=end
