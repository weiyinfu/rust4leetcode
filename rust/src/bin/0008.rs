use std::str::FromStr;
use std::cmp::max;

struct Solution;

const MIN: i64 = -(1i64 << 31);
const MAX: i64 = (1i64 << 31) - 1;

fn parseValue(a: &Vec<char>, ind: usize, flag: i64) -> i64 {
    let mut v = 0i64;
    let mut j = ind;
    while j < a.len() && a[j] >= '0' && a[j] <= '9' {
        let now = a[j] as i32 - '0' as i32;
        // println!("now={}", now);
        v = v * 10 + (now as i64);
        if v * flag > MAX {
            return MAX;
        }
        if v * flag < MIN {
            return MIN;
        }
        j += 1;
    }
    return v;
}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let a: Vec<char> = str.chars().collect();
        for i in 0..a.len() {
            if a[i] != ' ' {
                if a[i] == '+' {
                    return parseValue(&a, i + 1, 1) as i32;
                } else if a[i] == '-' {
                    return -parseValue(&a, i + 1, -1) as i32;
                } else if a[i] >= '0' && a[i] <= '9' {
                    return parseValue(&a, i, 1) as i32;
                } else {
                    return 0;
                }
            }
        }
        return 0;
        unreachable!();
    }
}

fn main() {
    let s = Solution::my_atoi("42".to_string());
    println!("{}", s);
}