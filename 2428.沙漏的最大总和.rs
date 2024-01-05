/*
 * @lc app=leetcode.cn id=2428 lang=rust
 *
 * [2428] 沙漏的最大总和
 */

// @lc code=start
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ret = 0;
        for i in 1..m-1 {
            for j in 1..n-1 {
                let tmp = grid[i][j] +
                            grid[i+1][j] +
                            grid[i-1][j] +
                            grid[i+1][j+1] +
                            grid[i-1][j-1] +
                            grid[i-1][j+1] +
                            grid[i+1][j-1];
                ret = ret.max(tmp);
            }
        }
        ret
    }
}
// @lc code=end

