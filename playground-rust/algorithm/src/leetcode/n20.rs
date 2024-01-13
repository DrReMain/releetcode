pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let s = s.as_bytes();
        if s.len() % 2 != 0 {
            return false;
        }

        let map = vec![(b')', b'('), (b'}', b'{'), (b']', b'[')]
            .into_iter()
            .collect::<HashMap<u8, u8>>();
        let mut stack: Vec<u8> = Vec::new();

        for b in s {
            if stack.len() > 0 && map.get(b).eq(&stack.last()) {
                stack.pop();
            } else {
                stack.push(*b);
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_is_valid1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn test_is_valid2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test_is_valid3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }

    #[test]
    fn test_is_valid4() {
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
    }

    #[bench]
    fn bench_is_valid(b: &mut Bencher) {
        b.iter(|| Solution::is_valid(String::from("()")));
    }
}
