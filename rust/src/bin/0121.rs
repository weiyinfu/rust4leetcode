use std::cmp::min;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len()==0{
            return 0;
        }
        let mut mi = prices[0];
        let mut ma = 0;
        for i in 1..prices.len() {
            let now = prices[i] - mi;
            if now > ma {
                ma = now;
            }
            mi = min(prices[i], mi);
        }
        ma
    }
}

fn main() {
    let ans=Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
    println!("{}",ans);
}