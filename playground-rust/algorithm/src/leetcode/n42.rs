pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);
        let mut ret = 0;
        while left < right {
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);

            if height[left] < height[right] {
                ret += left_max - height[left];
                left += 1;
            } else {
                ret += right_max - height[right];
                right -= 1;
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
    fn test_trap1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn test_trap2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }

    #[bench]
    fn bench_trap(b: &mut Bencher) {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        b.iter(|| Solution::trap(height.clone()));
    }
}
