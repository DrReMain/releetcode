pub struct Solution;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut ret = 0;
        for &n in set.iter() {
            if !set.contains(&(n - 1)) {
                let mut max = 1;
                let mut p = n;
                while set.contains(&(p + 1)) {
                    p += 1;
                    max += 1;
                }
                ret = ret.max(max);
            }
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
    fn test_longest_consecutive1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test_longest_consecutive2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }

    #[bench]
    fn bench_longest_consecutive(b: &mut Bencher) {
        let nums = vec![100, 4, 200, 1, 3, 2];
        b.iter(|| Solution::longest_consecutive(nums.clone()));
    }
}
