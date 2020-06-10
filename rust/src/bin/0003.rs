use std::collections::{HashMap};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut a = HashMap::<char, usize>::new();
        let mut i = 0usize;
        let mut ans = 0usize;
        let chars: Vec<char> = s.chars().collect();
        for j in 0..chars.len() {
            let c = &chars[j];
            if a.contains_key(c) {
                let pos = *a.get(c).unwrap();
                while i <= pos {
                    a.remove(&chars[i]);
                    i += 1;
                }
            }
            a.insert(*c, j);
            let len = j - i + 1;
            // println!("s[{}]={} len={} i={}", j, c, len, i);
            if ans < len {
                ans = len;
            }
        }
        return ans as i32;
    }
}

fn main() {
    let ans = Solution::length_of_longest_substring("a".to_string());
    println!("{}", ans);
}