pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        if n < 2 {
            return n as i32;
        }

        let mut m = HashMap::<char, usize>::new();
        let (mut ret, mut p) = (0, 0);
        for (i, c) in s.iter().enumerate() {
            if let Some(idx) = m.get(c) {
                p = p.max(idx + 1);
            }
            ret = ret.max(i + 1 - p);
            m.insert(*c, i);
        }
        ret as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_length_of_longest_substring1() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
    }

    #[test]
    fn test_length_of_longest_substring2() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
    }

    #[bench]
    fn bench_length_of_longest_substring(b: &mut Bencher) {
        b.iter(|| Solution::length_of_longest_substring(String::from("abcabcbb")));
    }
}
