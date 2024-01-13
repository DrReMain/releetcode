pub struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ret: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    ret.push(vec![nums[i], nums[left], nums[right]]);
                    while left < right && nums[left + 1] == nums[left] {
                        left += 1;
                    }
                    while left < right && nums[right - 1] == nums[right] {
                        right -= 1;
                    }
                    left += 1;
                    right -= 1;
                }
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
    fn test_three_sum1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(nums);

        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        for e in expected.iter() {
            assert_eq!(true, result.contains(e));
        }
        for r in result.iter() {
            assert_eq!(true, expected.contains(r));
        }
    }

    #[test]
    fn test_three_sum2() {
        let nums = vec![0, 1, 1];
        let result = Solution::three_sum(nums);

        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_sum3() {
        let nums = vec![0, 0, 0];
        let result = Solution::three_sum(nums);

        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }

    #[bench]
    fn bench_three_sum(b: &mut Bencher) {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        b.iter(|| Solution::three_sum(nums.clone()));
    }
}
