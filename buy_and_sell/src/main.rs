use std::cmp;
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        if prices.len() == 0 {}
        let mut max_profit: i32 = 0;
        let mut lowest_buy = prices[0];

        for p in prices {
            let possible_profit = p - lowest_buy;
            max_profit = cmp::max(max_profit, possible_profit);
            lowest_buy = cmp::min(lowest_buy, p);
        }
        return max_profit;
    }
}

fn main() {
    println!("{:?}", Solution::max_profit(vec![10,1,5,6,7,1]));
}