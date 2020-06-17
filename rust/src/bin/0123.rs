use std::cmp::max;

struct Solution;

fn f(p: &Vec<i32>, ind: usize, dp: &mut Vec<Vec<i32>>, cnt: i32) -> i32 {
    if ind == p.len() {
        return 0;
    }
    if dp[ind][cnt as usize] != -1 {
        return dp[ind][cnt as usize];
    }
    if cnt == 0 {
        return 0;
    }
    //如果不买
    let mut profit = f(p, ind + 1, dp, cnt);
    //如果买，就要决定卖的日期
    if cnt > 0 {
        for sell_day in ind + 1..p.len() {
            if p[sell_day] > p[ind] {
                let buy = f(p, sell_day, dp, cnt - 1) + (p[sell_day] - p[ind]);
                if buy > profit {
                    profit = buy;
                }
            }
        }
    }

    dp[ind][cnt as usize] = profit;
    profit
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len()==0{
            return 0;
        }
        let mut dp = vec!(vec!(-1; 3); prices.len());
        let ans = f(&prices, 0, &mut dp, 2);
        println!("{:?}", dp);
        dp[0][2]
    }
}

fn main() {
    let ans = Solution::max_profit(vec![1, 2, 3, 4, 5]);
    println!("{}", ans);
}