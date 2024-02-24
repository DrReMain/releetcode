pub struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut dict = HashMap::<i32, usize>::new();

        for (i, n) in nums.iter().enumerate() {
            if let Some(idx) = dict.get(n) {
                return vec![*idx as i32, i as i32];
            } else {
                dict.insert(target - n, i);
            }
        }

        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_two_sum1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[test]
    fn test_two_sum3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }

    #[bench]
    fn bench_two_sum(b: &mut Bencher) {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        b.iter(|| Solution::two_sum(nums.clone(), target));
    }
}
