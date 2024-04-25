/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 */

// @lc code=start
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
// @lc code=end
