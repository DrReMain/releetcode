pub struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut left, mut right, mut ret) = (0, 0, 0);
        for &c in s.iter() {
            if c == '(' {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                ret = ret.max(right * 2);
            } else if right > left {
                (left, right) = (0, 0);
            }
        }
        (left, right) = (0, 0);
        for &c in s.iter().rev() {
            if c == ')' {
                right += 1;
            } else {
                left += 1;
            }
            if left == right {
                ret = ret.max(left * 2);
            } else if left > right {
                (left, right) = (0, 0);
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
    fn test_longest_valid_parentheses1() {
        let s = String::from("(()");
        let expected = 2;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
    }

    #[test]
    fn test_longest_valid_parentheses2() {
        let s = String::from(")()())");
        let expected = 4;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
    }

    #[test]
    fn test_longest_valid_parentheses3() {
        let s = String::from("");
        let expected = 0;
        assert_eq!(Solution::longest_valid_parentheses(s), expected);
    }

    #[bench]
    fn bench_longest_valid_parentheses(b: &mut Bencher) {
        let s = String::from("(()");
        b.iter(|| Solution::longest_valid_parentheses(s.clone()));
    }
}
