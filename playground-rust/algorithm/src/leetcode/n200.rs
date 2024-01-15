pub struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        if grid.len() == 0 {
            return 0;
        }
        let (row, col) = (grid.len(), grid[0].len());
        let mut islands = 0;
        for i in 0..row {
            for j in 0..col {
                if grid[i][j] == '1' {
                    islands += 1;
                    Self::num_islands_dfs(&mut grid, i as i32, j as i32);
                }
            }
        }
        islands
    }
    pub fn num_islands_dfs(grid: &mut Vec<Vec<char>>, i: i32, j: i32) {
        let (row, col) = (grid.len() as i32, grid[0].len() as i32);
        if i < 0 || j < 0 || i >= row || j >= col || grid[i as usize][j as usize] == '0' {
            return;
        }
        grid[i as usize][j as usize] = '0';
        Self::num_islands_dfs(grid, i + 1, j);
        Self::num_islands_dfs(grid, i - 1, j);
        Self::num_islands_dfs(grid, i, j + 1);
        Self::num_islands_dfs(grid, i, j - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_num_islands1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[bench]
    fn bench_num_islands(b: &mut Bencher) {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        b.iter(|| Solution::num_islands(grid.clone()));
    }
}
