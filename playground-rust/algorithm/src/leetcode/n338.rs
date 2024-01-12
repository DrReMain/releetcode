pub struct Solution;
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        vec![0; n as usize + 1]
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let (mut count, mut remain) = (0, idx as i32);
                while remain > 0 {
                    if remain % 2 != 0 {
                        count += 1;
                    }
                    remain >>= 1;
                }
                count
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_count_bits() {
        let tests = vec![(2, vec![0, 1, 1]), (5, vec![0, 1, 1, 2, 1, 2])];
        for (n, expected) in tests {
            assert_eq!(Solution::count_bits(n), expected);
        }
    }

    #[bench]
    fn bench_count_bits(b: &mut Bencher) {
        b.iter(|| Solution::count_bits(1000));
    }
}
