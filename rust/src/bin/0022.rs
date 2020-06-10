struct Solution;

fn go(a: &mut Vec<String>, s: &mut String, sta: &mut Vec<char>, n: i32) {
    if n == 0 && sta.is_empty() {
        a.push(s.clone());
        return;
    }
    //添加左括号
    if n > 0 {
        s.push('(');
        sta.push('(');
        go(a, s, sta, n - 1);
        sta.pop();
        s.pop();
    }
    //添加右括号
    let last = sta.last();
    if last.is_some() {
        let ch = last.unwrap();
        if *ch == '(' {
            s.push(')');
            sta.pop();
            go(a, s, sta, n);
            sta.push('(');
            s.pop();
        }
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut a = Vec::<String>::new();
        let mut s = "".to_string();
        let mut sta = Vec::<char>::new();
        go(&mut a, &mut s, &mut sta, n);
        a
    }
}

fn main() {
    let ans = Solution::generate_parenthesis(1);
    println!("{:?}", ans);
}