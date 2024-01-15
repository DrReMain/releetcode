pub struct Solution;
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; n as usize]; n as usize];
        let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut row, mut col, mut d) = (0i32, 0i32, 0);
        for i in 1..=n * n {
            ret[row as usize][col as usize] = i;
            let (next_row, next_col) = (row + dirs[d].0, col + dirs[d].1);
            if next_row < 0
                || next_col < 0
                || next_row >= n
                || next_col >= n
                || ret[next_row as usize][next_col as usize] > 0
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
    fn test_generate_matrix1() {
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(Solution::generate_matrix(3), expected);
    }

    #[test]
    fn test_generate_matrix2() {
        let expected = vec![vec![1]];
        assert_eq!(Solution::generate_matrix(1), expected);
    }

    #[bench]
    fn bench_generate_matrix(b: &mut Bencher) {
        b.iter(|| Solution::generate_matrix(3));
    }
}
