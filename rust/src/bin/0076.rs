use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() == 0 {
            return "".to_string();
        }
        let mut ma = HashMap::new();
        let mut i = 0usize;//左侧指针
        for i in t.chars() {
            let now = if ma.contains_key(&i) { *ma.get(&i).unwrap() } else { 0 };
            ma.insert(i, now + 1);
        }
        let mut cnt = 0;
        let mut ans: (i32, i32) = (-1, -1);
        let a: Vec<char> = s.chars().collect();
        for j in 0..a.len() {
            if !ma.contains_key(&a[j]) {
                continue;
            }
            let now = *ma.get(&a[j]).unwrap();
            ma.insert(a[j], now - 1);
            if now == 1 {
                cnt += 1;
            }
            // println!("j={} ma={:?} see char {} i={} cnt={}", j, ma, a[j], i, cnt);
            loop {
                if !ma.contains_key(&a[i]) {
                    i += 1;
                    continue;
                }
                let now = *ma.get(&a[i]).unwrap();
                if now < 0 {
                    ma.insert(a[i], now + 1);
                    i += 1;
                } else {
                    break;
                }
            }

            if cnt == ma.len() {
                if ans.0 == -1 || (j as i32 - i as i32) < ans.1 - ans.0 {
                    ans = (i as i32, j as i32);
                }
            }
        }
        if ans.0 == -1 {
            return "".to_string();
        }
        s[ans.0 as usize..=ans.1 as usize].to_string()
    }
}

fn main() {
    let ans = Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string());
    println!("{}", ans);
}

