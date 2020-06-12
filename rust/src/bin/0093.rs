struct Solution;

fn go(ans: &mut Vec<String>, ind: usize, s: &Vec<i32>, now: &mut Vec<i32>) {
    if ind == s.len() {
        if now.len() != 4 {
            return;
        }
        let mut n = String::new();
        let mut is_first = true;
        for i in now {
            if is_first {
                is_first = false;
            } else {
                n.push('.');
            }
            n.push_str(i.to_string().as_str());
        }
        ans.push(n);
        return;
    }
    if now.len() >= 4 {
        return;
    }
    let mut ip = 0;
    for i in 0..3 {
        if ind + i >= s.len() {
            break;
        }
        ip = ip * 10 + s[ind + i];
        if ip > 255 {
            break;
        }
        now.push(ip);
        go(ans, ind + i + 1, s, now);
        now.pop();
        if ip == 0 && i == 0 {
            //不可能以0开头
            break;
        }
    }
}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = vec!();
        let mut ss = vec!();
        for i in s.chars() {
            ss.push(i as i32 - '0' as i32);
        }
        println!("{:?}", ss);
        let mut now = vec!();
        go(&mut ans, 0, &ss, &mut now);
        ans
    }
}

fn main() {
    let ans = Solution::restore_ip_addresses("1111".to_string());
    println!("{:?}", ans);
}