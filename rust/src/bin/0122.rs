use std::cmp::max;

struct Solution;

fn f(p: &Vec<i32>, ind: usize, dp: &mut Vec<i32>) -> i32 {
    if ind == p.len() {
        return 0;
    }
    if dp[ind] != -1 {
        return dp[ind];
    }
    //如果不买
    let mut profit = f(p, ind + 1, dp);
    let mut sell_day = ind + 1;
    while sell_day < p.len() && p[sell_day] <= p[ind] {
        sell_day += 1;
    }
    if sell_day < p.len() {
        let buy = f(p, sell_day, dp) + (p[sell_day] - p[ind]);
        if buy > profit {
            profit = buy;
        }
    }
    dp[ind] = profit;
    profit
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec!(-1; prices.len());
        f(&prices, 0, &mut dp)
    }
}

fn main() {
    let ans = Solution::max_profit(vec![1, 2, 3, 4, 5]);
    println!("{}", ans);
}
