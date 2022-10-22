pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 { return 0; }
        use std::cmp;
        let mut dp: Vec<i32> = vec![0; prices.len()];
        dp[0] = 0;
        dp[1] = cmp::max(prices[1] - prices[0], 0);
        let mut max_profit = cmp::max(dp[0], dp[1]);
        let _tmp = prices.iter().enumerate().map(|(i, c)| {
            if i >= 2 {
                dp[i] = cmp::max(prices[i] - prices[i - 1] + dp[i - 1], 0);
                if dp[i] > max_profit { max_profit = dp[i]; }
            }
            *c
        }).collect::<Vec<i32>>();
        max_profit
    }
}