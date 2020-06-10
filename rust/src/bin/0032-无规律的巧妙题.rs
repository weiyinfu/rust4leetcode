struct Solution;


use std::cmp::{max, min};
use std::iter::FromIterator;

fn go(s: &String) -> i32 {
    let mut ans = 0;
    let ss = s.chars().collect::<Vec<char>>();
    let mut left = 0;//左括号的个数
    let mut cur_right = 0;//遇到的右括号的个数
    for i in 0..s.len() {
        let ch = ss[i];
        if ch == '(' {
            //肯定入栈没得说
            left += 1;
        } else {
            //肯定弹栈，判断一下
            left -= 1;//正常弹栈
            cur_right += 1;
            if left < 0 {
                //出错了
                cur_right = 0;//右括号个数清0
                left = 0;
            }
            if left == 0 {
                ans = max(cur_right * 2, ans);
            }
        }
    }
    ans
}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let x = go(&s);
        let mut ss: Vec<char> = s.chars().collect();
        ss.reverse();
        for i in &mut ss {
            *i = if *i == '(' { ')' } else { '(' };
        }
        let y = go(&String::from_iter(ss));
        max(x, y)
    }
}

fn main() {
    let ans = Solution::longest_valid_parentheses(")()())".to_string());
    println!("{}", ans);
}

