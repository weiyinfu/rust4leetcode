struct Solution;

fn solve(ans: &mut Vec<i32>, a: &Vec<char>, ind: usize) -> i32 {
    if ind == a.len() {
        return 1;
    }
    if ans[ind] != -1 {
        return ans[ind];
    }
    let mut res = 0;
    let now = a[ind] as i32 - '0' as i32;
    if now == 0 {
        ans[ind] = 0;
        return 0;//没有0这种东西，下标从1开始
    }
    res += solve(ans, a, ind + 1);
    if ind + 1 < a.len() {
        let next = a[ind + 1] as i32 - '0' as i32;
        let k = now * 10 + next;
        if 1 <= k && k <= 26 {
            res += solve(ans, a, ind + 2);
        }
    }
    ans[ind] = res;
    res
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let a: Vec<char> = s.chars().collect();
        let mut ans = vec!(-1; a.len());
        let s = solve(&mut ans, &a, 0);
        s
    }
}

fn main() {
    let ans = Solution::num_decodings("12".to_string());
    println!("{}", ans);
}