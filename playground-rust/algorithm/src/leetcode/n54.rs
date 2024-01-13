pub struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut row, mut col, mut d) = (0i32, 0i32, 0);
        let mut visited = vec![vec![false; cols]; rows];
        let mut ret = vec![0; rows * cols];
        for i in 0..rows * cols {
            ret[i] = matrix[row as usize][col as usize];
            visited[row as usize][col as usize] = true;
            let (next_row, next_col) = (row + dirs[d].0, col + dirs[d].1);

            if next_row < 0
                || next_col < 0
                || next_row >= rows as i32
                || next_col >= cols as i32
                || visited[next_row as usize][next_col as usize]
            {
                d = (d + 1) % dirs.len();
            }

            row += dirs[d].0;
            col += dirs[d].1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_spiral_order1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(Solution::spiral_order(matrix), expected);
    }

    #[test]
    fn test_spiral_order2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(Solution::spiral_order(matrix), expected);
    }

    #[bench]
    fn bench_spiral_order(b: &mut Bencher) {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        b.iter(|| Solution::spiral_order(matrix.clone()));
    }
}
