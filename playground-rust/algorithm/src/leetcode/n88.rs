pub struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut p, mut x, mut y) = (nums1.len() - 1, m as usize, n as usize);
        while x.checked_sub(1).is_some() && y.checked_sub(1).is_some() {
            nums1[p] = if nums1[x - 1] >= nums2[y - 1] {
                x -= 1;
                nums1[x]
            } else {
                y -= 1;
                nums2[y]
            };
            p -= 1;
        }
        for i in 0..y {
            nums1[i] = nums2[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_merge1() {
        let (mut nums1, m) = (vec![1, 2, 3, 0, 0, 0], 3);
        let (mut nums2, n) = (vec![2, 5, 6], 3);
        let expected = vec![1, 2, 2, 3, 5, 6];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }

    #[test]
    fn test_merge2() {
        let (mut nums1, m) = (vec![1], 1);
        let (mut nums2, n) = (vec![], 0);
        let expected = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }

    #[test]
    fn test_merge3() {
        let (mut nums1, m) = (vec![0], 0);
        let (mut nums2, n) = (vec![1], 1);
        let expected = vec![1];
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, expected);
    }

    #[bench]
    fn bench_merge(b: &mut Bencher) {
        let (mut nums1, m) = (vec![1, 2, 3, 0, 0, 0], 3);
        let (mut nums2, n) = (vec![2, 5, 6], 3);
        b.iter(|| Solution::merge(&mut nums1, m, &mut nums2, n));
    }
}
