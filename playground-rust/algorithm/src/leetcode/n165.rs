pub struct Solution;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1 = version1.as_bytes();
        let version2 = version2.as_bytes();
        let (mut x, mut y) = (0usize, 0usize);
        while x < version1.len() || y < version2.len() {
            let (mut n1, mut n2) = (0, 0);
            while x < version1.len() && version1[x] != b'.' {
                n1 = n1 * 10 + (version1[x] - b'0') as i32;
                x += 1;
            }
            while y < version2.len() && version2[y] != b'.' {
                n2 = n2 * 10 + (version2[y] - b'0') as i32;
                y += 1;
            }
            if n1 > n2 {
                return 1;
            } else if n1 < n2 {
                return -1;
            }

            x += 1;
            y += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_compare_version1() {
        assert_eq!(
            Solution::compare_version(String::from("1.01"), String::from("1.001")),
            0
        );
    }

    #[test]
    fn test_compare_version2() {
        assert_eq!(
            Solution::compare_version(String::from("1.0"), String::from("1.0.0")),
            0
        );
    }

    #[test]
    fn test_compare_version3() {
        assert_eq!(
            Solution::compare_version(String::from("0.1"), String::from("1.1")),
            -1
        );
    }

    #[test]
    fn test_compare_version4() {
        assert_eq!(
            Solution::compare_version(String::from("123"), String::from("321")),
            -1
        );
    }

    #[bench]
    fn bench_compare_version(b: &mut Bencher) {
        b.iter(|| Solution::compare_version(String::from("1.01"), String::from("1.001")));
    }
}
