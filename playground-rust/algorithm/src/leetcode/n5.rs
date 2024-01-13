pub struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let (mut start, mut end) = (0, 0);
        for i in 0..s.len() {
            let (left1, right1) = Self::longest_palindrome_eac(&s, i as i32, i as i32);
            let (left2, right2) = Self::longest_palindrome_eac(&s, i as i32, i as i32 + 1);
            if right1 - left1 > end - start {
                (start, end) = (left1, right1);
            }
            if right2 - left2 > end - start {
                (start, end) = (left2, right2);
            }
        }
        s[start as usize..=end as usize].iter().collect()
    }
    pub fn longest_palindrome_eac(s: &Vec<char>, mut left: i32, mut right: i32) -> (i32, i32) {
        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        (left + 1, right - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_longest_palindrome1() {
        let s = "babad".to_string();
        let expected = vec!["bab".to_string(), "aba".to_string()];

        assert!(expected.contains(&Solution::longest_palindrome(s)));
    }

    #[test]
    fn test_longest_palindrome2() {
        let s = "cbbd".to_string();
        let expected = "bb".to_string();

        assert_eq!(Solution::longest_palindrome(s), expected);
    }

    #[bench]
    fn bench_longest_palindrome(b: &mut Bencher) {
        b.iter(|| Solution::longest_palindrome(String::from("babad")));
    }
}
