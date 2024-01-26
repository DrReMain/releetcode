pub struct Solution;
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cur = 0usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[cur] = nums[i];
                cur += 1;
            }
        }
        cur as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_remove_element1() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
    }

    #[test]
    fn test_remove_element2() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), 5);
    }

    #[bench]
    fn bench_remove_element(b: &mut Bencher) {
        let nums = vec![3, 2, 2, 3];
        let val = 3;
        b.iter(|| Solution::remove_element(&mut nums.clone(), val));
    }
}
