pub struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        nums.iter().fold(nums[0], |ret, &n| {
            prev = n.max(prev + n);
            ret.max(prev)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_max_sub_array1() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn test_max_sub_array2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn test_max_sub_array3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }

    #[bench]
    fn bench_max_sub_array(b: &mut Bencher) {
        b.iter(|| Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }
}
