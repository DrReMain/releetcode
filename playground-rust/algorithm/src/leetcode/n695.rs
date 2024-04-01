pub struct Solution;
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_max_area_of_island1() {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }

    #[test]
    fn test_max_area_of_island2() {
        let grid = vec![vec![0, 0, 0, 0, 0, 0, 0, 0]];
        assert_eq!(Solution::max_area_of_island(grid), 0);
    }

    #[bench]
    fn bench_max_area_of_island(b: &mut Bencher) {
        let grid = vec![
            vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ];
        b.iter(|| Solution::max_area_of_island(grid.clone()));
    }
}
