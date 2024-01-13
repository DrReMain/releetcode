pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        prices.into_iter().fold(0, |income, p| {
            let new_income = income.max(p - buy);
            buy = buy.min(p);
            new_income
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use test::Bencher;

    #[test]
    fn test_max_profit1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5);
    }

    #[test]
    fn test_max_profit2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[bench]
    fn bench_max_profit(b: &mut Bencher) {
        let prices = vec![7, 1, 5, 3, 6, 4];
        b.iter(|| Solution::max_profit(prices.clone()));
    }
}
