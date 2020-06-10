use std::cmp::max;

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut x: Vec<i32> = a.chars().map(|x| { if x == '0' { 0 } else { 1 } }).collect();
        let mut y: Vec<i32> = b.chars().map(|x| if x == '0' { 0 } else { 1 }).collect();
        x.reverse();
        y.reverse();
        let mut ans = vec!();
        let mut enter = 0;
        for i in 0..max(x.len(), y.len()) + 1 {
            let xx = if i >= x.len() { 0 } else { x[i] };
            let yy = if i >= y.len() { 0 } else { y[i] };
            let mut now = xx + yy + enter;
            enter = now / 2;
            now %= 2;
            ans.push(now);
        }
        let mut i = ans.len() as i32 - 1;
        // println!("{:?}", ans);
        while i >= 0 {
            if ans[i as usize] == 0 {
                ans.pop();
            } else {
                break;
            }
            i -= 1;
        }
        // println!("{}", i);
        if i == -1 {
            return "0".to_string();
        }
        let mut s = String::new();
        ans.reverse();
        for i in ans {
            s.push_str(i.to_string().as_str());
        }
        s
    }
}

fn main() {
    let ans = Solution::add_binary("0110".to_string(), "00111".to_string());
    println!("{}", ans);
}