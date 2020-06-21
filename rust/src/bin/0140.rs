use std::iter::FromIterator;

struct Solution;

fn same(s: &[char], x: &[char]) -> bool {
    for i in 0..s.len() {
        if s[i] != x[i] {
            return false;
        }
    }
    return true;
}

fn go(dp: &mut Vec<Option<Vec<String>>>, s: &Vec<char>, words: &Vec<Vec<char>>,
      ind: usize) -> Vec<String> {
    if dp[ind].is_some() {
        return dp[ind].as_ref().unwrap().clone();
    }
    if ind == s.len() {
        return vec!("".to_string());
    }
    let mut now = vec!();
    for i in words {
        if ind + i.len() <= s.len() {
            let ss = &s[ind..ind + i.len()];
            let w = &i[..];
            if same(&ss, &w) {
                let ans = go(dp, s, words, ind + i.len());
                for son in ans {
                    let mut p = String::from_iter(i);

                    if son.len() >0{
                        p.push(' ');
                        p.push_str(son.as_str());
                    }
                    now.push(p);
                }
            }
        }
    }
    dp[ind] = Some(now);
    dp[ind].as_ref().unwrap().clone()
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let words: Vec<Vec<char>> = word_dict.iter().map(|x| { x.chars().collect::<Vec<char>>() }).collect();
        let mut dp = vec!(None; s.len() + 1);
        go(&mut dp, &s, &words, 0);
        dp[0].as_ref().unwrap().clone()
    }
}

fn main() {
    let s = "catsanddog".to_string();
    let wordDict = vec![
        "cat".to_string(),
        "cats".to_string(),
        "and".to_string(),
        "sand".to_string(),
        "dog".to_string()];
    let ans = Solution::word_break(s, wordDict);
    println!("{:?}", ans);
}