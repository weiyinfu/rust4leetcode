struct Solution;

use std::cmp::max;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let ans: Vec<&str> = s.split_whitespace().collect();
        let la = ans.last();
        return if la.is_none() { 0 } else { la.unwrap().len() as i32 };
    }
}

fn main() {
    let ans = Solution::length_of_last_word("hello wlrd".to_string());
    println!("{}", ans);
}