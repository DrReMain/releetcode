pub struct Solution;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let length = nums.len();
        let mut q = Vec::<usize>::new();
        for i in 0..k {
            while q.len() > 0 && nums[i] >= nums[q[q.len() - 1]] {
                q.pop();
            }
            q.push(i);
        }

        let mut ret = vec![nums[q[0]]];
        for i in k..length {
            while q.len() > 0 && nums[i] >= nums[q[q.len() - 1]] {
                q.pop();
            }
            q.push(i);

            while q[0] <= i - k {
                // q = q[1..].to_vec();
                q = q.drain(1..).collect();
            }
            ret.push(nums[q[0]]);
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
    fn test_max_sliding_window1() {
        let (nums, target) = (vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        let expected = vec![3, 3, 5, 5, 6, 7];
        assert_eq!(Solution::max_sliding_window(nums, target), expected);
    }

    #[test]
    fn test_max_sliding_window2() {
        let (nums, target) = (vec![1], 1);
        let expected = vec![1];
        assert_eq!(Solution::max_sliding_window(nums, target), expected);
    }

    #[bench]
    fn bench_max_sliding_window(b: &mut Bencher) {
        let (nums, target) = (vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        b.iter(|| Solution::max_sliding_window(nums.clone(), target));
    }
}
