struct Solution;

fn indexOf(s: &str, i: char) -> i32 {
    let mut j = 0;
    for ch in s.chars() {
        if i == ch {
            return j;
        }
        j += 1;
    }
    -1
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut a = Vec::<char>::new();
        let left = "[{(";
        let right = "]})";
        let s = s.chars().collect::<Vec<char>>();
        for i in s {
            if indexOf(left, i) != -1 {
                a.push(i);
                continue;
            }
            //如果不是左括号
            let ind: i32 = indexOf(right, i);
            if a.is_empty() {
                return false;
            }
            let last_char = a.last().unwrap();
            let last_ind = indexOf(left, *last_char);
            if last_ind == -1 || last_ind != ind {
                return false;
            }
            a.pop();
        }
        a.is_empty()
    }
}

fn main() {
    let ans = Solution::is_valid("(}".to_string());
    println!("{}", ans);
}