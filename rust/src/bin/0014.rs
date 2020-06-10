use std::str::Chars;

struct Solution;


impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let mut s = "".to_string();
        let mut a = Vec::<Vec<char>>::new();
        for i in strs {
            a.push(i.chars().collect());
        }
        let mut j = 0;
        'out:
        loop {
            let mut cur: Option<char> = None;
            for i in &a {
                if j >= i.len() {
                    break 'out;
                }
                let now = i[j];
                if cur.is_some() && now != cur.unwrap() {
                    break 'out;
                } else {
                    cur = Some(now);
                }
            }
            s.push(cur.unwrap());
            j += 1;
        }
        s
    }
}

fn gets(a: Vec<&str>) -> Vec<String> {
    let mut b = Vec::<String>::new();
    for i in a {
        b.push(i.to_string());
    }
    b
}

fn main() {
    println!("{}", Solution::longest_common_prefix(gets(vec!["flower", "flow", "flight"])));
}