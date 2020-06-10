use std::collections::HashMap;
use std::ops::{Add, AddAssign};

struct Solution;

fn get(n: i32, one: char, five: char, ten: char) -> String {
    if n <= 3 {
        return one.to_string().repeat(n as usize);
    } else if n == 4 {
        let mut x = one.to_string();
        x.push(five);
        return x;
    } else if n < 9 {
        let mut x = five.to_string();
        for i in 6..=n {
            x.push(one);
        }
        return x;
    } else if n == 9 {
        let mut s = one.to_string();
        s.push(ten);
        return s;
    }
    unreachable!()
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut x = num;
        let m = [
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
        ];
        let mut ans = "".to_string();
        ans.add_assign(&get(x / 1000, 'M', 'Z', 'W'));
        x %= 1000;
        ans.add_assign(&get(x / 100, 'C', 'D', 'M'));
        x %= 100;
        ans.add_assign(&get(x / 10, 'X', 'L', 'C'));
        x %= 10;
        ans.add_assign(&get(x, 'I', 'V', 'X'));
        ans
    }
}

fn main() {
    println!("{}", Solution::int_to_roman(3));
}