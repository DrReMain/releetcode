pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut ret = vec![1; length];
        for i in 1..length {
            ret[i] = ret[i-1] * nums[i-1];
        }

        let mut p = 1;
        for i in (0..length).rev() {
            ret[i] = ret[i] * p;
            p *= nums[i];
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
    fn test_product_except_self1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[test]
    fn test_product_except_self2() {
        let nums = vec![-1,1,0,-3,3];
        let expected = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[bench]
    fn bench_product_except_self(b: &mut Bencher) {
        let nums = vec![1, 2, 3, 4];
        b.iter(|| Solution::product_except_self(nums.clone()));
    }
}
