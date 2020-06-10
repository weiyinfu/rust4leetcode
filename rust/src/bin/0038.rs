struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = "1".to_string();
        for i in 2..=n {
            let mut a = String::new();
            let mut cnt = 0;
            let mut last = 'a';
            for ch in s.chars() {
                if ch == last {
                    cnt += 1;
                } else {
                    if cnt > 0 {
                        a += cnt.to_string().as_str();
                        a.push(last);
                    }
                    cnt = 1;
                    last = ch;
                }
            }
            if cnt > 0 {
                a += cnt.to_string().as_str();
                a.push(last);
            }
            s = a;
            println!("{} {}", i, s);
        }
        s
    }
}

fn main() {
    Solution::count_and_say(10);
}