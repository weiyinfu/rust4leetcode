struct Solution;

fn s2i(s: &String) -> Vec<i32> {
    let mut a = vec!();
    for i in s.chars() {
        a.push(i as i32 - '0' as i32);
    }
    a
}

fn i2s(a: &Vec<i32>) -> String {
    let mut s = String::new();
    let mut i = (a.len() - 1) as i32;
    while i >= 0 && a[i as usize] == 0 {
        i -= 1;
    }
    if i == -1 {
        return "0".to_string();
    }
    let mut j = i;
    while j >= 0 {
        s.push((a[j as usize] + '0' as i32) as u8 as char);
        j -= 1;
    }
    s
}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut a = s2i(&num1);
        let mut b = s2i(&num2);
        a.reverse();
        b.reverse();
        let mut c = vec!();
        for i in 0..1 + a.len() + b.len() {
            c.push(0);
        }
        for i in 0..a.len() {
            for j in 0..b.len() {
                c[i + j] += a[i] * b[j];
            }
        }
        let mut enter = 0;
        for i in 0..c.len() {
            c[i] += enter;
            enter = c[i] / 10;
            c[i] %= 10;
        }
        let ans = i2s(&c);
        ans
    }
}

fn main() {
    let ans = Solution::multiply("123".to_string(), "0".to_string());
    println!("{}", ans);
}