pub struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            dp[i][0] = 1;
        }
        for i in 0..n {
            dp[0][i] = 1;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
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
    fn test_unique_paths1() {
        let (m, n) = (3, 7);
        let expected = 28;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_unique_paths2() {
        let (m, n) = (3, 2);
        let expected = 3;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_unique_paths3() {
        let (m, n) = (7, 3);
        let expected = 28;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[test]
    fn test_unique_paths4() {
        let (m, n) = (3, 3);
        let expected = 6;
        assert_eq!(Solution::unique_paths(m, n), expected);
    }

    #[bench]
    fn bench_unique_paths(b: &mut Bencher) {
        let (m, n) = (3, 7);
        b.iter(|| Solution::unique_paths(m, n));
    }
}
