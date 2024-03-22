pub struct Solution;
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        nums.iter().enumerate().fold(0, |ret, (idx, &next)| {
            if next > nums[ret as usize] {
                idx as i32
            } else {
                ret
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_compare_version1() {
        let nums = vec![1, 2, 3, 1];
        let expected = 2;
        assert_eq!(Solution::find_peak_element(nums), expected);
    }

    #[test]
    fn test_compare_version2() {
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let expected = 5;
        assert_eq!(Solution::find_peak_element(nums), expected);
    }

    #[bench]
    fn bench_compare_version(b: &mut Bencher) {
        let nums = vec![1, 2, 3, 1];
        b.iter(|| Solution::find_peak_element(nums.clone()));
    }
}
