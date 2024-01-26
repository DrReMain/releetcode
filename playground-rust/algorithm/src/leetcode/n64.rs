pub struct Solution;
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_min_path_sum1() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[bench]
    fn bench_min_path_sum(b: &mut Bencher) {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        b.iter(|| Solution::min_path_sum(grid.clone()));
    }
}
