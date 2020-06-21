use std::iter::FromIterator;

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

fn go(ans: &mut Vec<Vec<String>>, now: &mut Vec<String>, s: &[char], ind: usize) {
    if ind == s.len() {
        ans.push(now.clone());
        return;
    }
    for j in ind + 1..=s.len() {
        if !is(&s[ind..j]) {
            continue;
        }
        now.push(String::from_iter(s[ind..j].iter()));
        go(ans, now, s, j);
        now.pop();
    }
}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let ss: Vec<char> = s.chars().collect();
        let mut ans = vec!();
        let mut now = vec![];
        go(&mut ans, &mut now, &ss, 0);
        ans
    }
}

fn main() {
    let a = Solution::partition("aab".to_string());
    println!("{:?}", a);
}