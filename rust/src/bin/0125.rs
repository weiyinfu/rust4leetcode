struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s.to_lowercase().chars().filter(|x| *x >= '0' && *x <= '9' || *x >= 'a' && *x <= 'z').collect();
        let a: String = s.chars().rev().collect();
        a == s
    }
}

fn main() {
    let res = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());
    println!("{}", res);
}