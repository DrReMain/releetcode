/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 */

// @lc code=start
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            dp[i][0] = if i > 0 {
                grid[i][0] + dp[i - 1][0]
            } else {
                grid[i][0]
            };
        }
        for i in 1..n {
            dp[0][i] = grid[0][i] + dp[0][i - 1];
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}
// @lc code=end

