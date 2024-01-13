pub struct Solution;
impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let (mut x, mut y, mut carry) = (num1.len(), num2.len(), 0);
        let mut ret = String::new();
        while x.checked_sub(1).is_some() || y.checked_sub(1).is_some() || carry > 0 {
            let n1 = if let Some(idx) = x.checked_sub(1) {
                x = idx;
                (num1[idx] - b'0') as i32
            } else {
                0
            };
            let n2 = if let Some(idx) = y.checked_sub(1) {
                y = idx;
                (num2[idx] - b'0') as i32
            } else {
                0
            };

            let sum = n1 + n2 + carry;
            carry = sum / 10;

            ret = (sum % 10).to_string() + ret.as_str();
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
    fn test_add_strings1() {
        let num1 = "456".to_string();
        let num2 = "77".to_string();
        assert_eq!(Solution::add_strings(num1, num2), "533".to_string());
    }

    #[test]
    fn test_add_strings2() {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        assert_eq!(Solution::add_strings(num1, num2), "134".to_string());
    }

    #[test]
    fn test_add_strings3() {
        let num1 = "0".to_string();
        let num2 = "0".to_string();
        assert_eq!(Solution::add_strings(num1, num2), "0".to_string());
    }

    #[bench]
    fn bench_add_strings(b: &mut Bencher) {
        let num1 = "11".to_string();
        let num2 = "123".to_string();
        b.iter(|| Solution::add_strings(num1.clone(), num2.clone()));
    }
}
