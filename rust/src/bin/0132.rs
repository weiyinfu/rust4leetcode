use std::cmp::min;

struct Solution;

fn is(s: &[char]) -> bool {
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    return true;
}

fn go(s: &[char], dp: &mut Vec<i32>, ind: usize) -> i32 {
    if dp[ind] != -1 {
        return dp[ind];
    }
    if ind == s.len() {
        return 0;
    }
    let mut best = 1 << 30;
    for j in ind + 1..=s.len() {
        if is(&s[ind..j]) {
            let now = go(s, dp, j);
            best = min(now + 1, best);
        }
    }
    dp[ind] = best;
    best
}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut dp = vec!(-1; s.len() + 1);
        let ans = go(&s, &mut dp, 0);
        println!("{:?}", dp);
        ans-1
    }
}

fn main() {
    let ans = Solution::min_cut("aab".to_string());
    println!("{}", ans);
}