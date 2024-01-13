pub struct Solution;
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        Self::permute_backtrack(&mut ret, &nums, &mut Vec::new());
        ret
    }
    pub fn permute_backtrack(ret: &mut Vec<Vec<i32>>, nums: &Vec<i32>, path: &mut Vec<i32>) {
        if path.len() == nums.len() {
            ret.push(path.clone());
            return;
        }

        for n in nums {
            if path.contains(n) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(*n);
            Self::permute_backtrack(ret, nums, &mut new_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_permute1() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute(nums.clone());

        let expected_results: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];

        for expected in expected_results.iter() {
            assert_eq!(true, result.contains(expected));
        }
        for r in result.iter() {
            assert_eq!(true, expected_results.contains(r));
        }
    }

    #[test]
    fn test_permute2() {
        let nums = vec![0, 1];
        let result = Solution::permute(nums.clone());

        let expected_results: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];

        for expected in expected_results.iter() {
            assert_eq!(true, result.contains(expected));
        }
        for r in result.iter() {
            assert_eq!(true, expected_results.contains(r));
        }
    }

    #[test]
    fn test_permute3() {
        let nums = vec![1];
        let result = Solution::permute(nums.clone());

        let expected_results: Vec<Vec<i32>> = vec![vec![1]];

        for expected in expected_results.iter() {
            assert_eq!(true, result.contains(expected));
        }
        for r in result.iter() {
            assert_eq!(true, expected_results.contains(r));
        }
    }

    #[bench]
    fn bench_permute(b: &mut Bencher) {
        let nums = vec![1, 2, 3];
        b.iter(|| Solution::permute(nums.clone()));
    }
}
