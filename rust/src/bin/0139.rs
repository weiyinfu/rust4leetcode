struct Solution;

fn same(a: &[char], b: &Vec<char>) -> bool {
    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}

fn go(dp: &mut [i32], s: &Vec<char>, words: &Vec<Vec<char>>, ind: usize) -> bool {
    if dp[ind] != -1 {
        return dp[ind] == 1;
    }
    if ind == s.len() {
        return true;
    }
    let mut ans = false;
    for i in words.iter() {
        if ind + i.len() <= s.len() && same(&s[ind..i.len() + ind], i) {
            let now = go(dp, s, words, ind + i.len());
            if now {
                ans = true;
                break;
            }
        }
    }
    dp[ind] = if ans { 1 } else { 0 };
    ans
}

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut can = vec![-1; s.len() + 1];
        let s: Vec<char> = s.chars().collect();
        let words = word_dict.iter().map(|x| {
            x.chars().collect::<Vec<char>>()
        }).collect();
        let ans = go(&mut can, &s, &words, 0);
        println!("{:?}", can);
        ans
    }
}

fn main() {
    let s = "catsandog".to_string();
    let words = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string()
    ];
    let ans = Solution::word_break(s, words);
    println!("{}", ans);
}