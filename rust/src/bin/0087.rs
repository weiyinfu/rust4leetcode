use std::collections::{HashSet, HashMap};

struct Solution;

fn go(a: Vec<char>, b: Vec<char>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    if a.len() == 0 {
        return true;
    }
    if a.len() == 1 {
        //两个一定可分
        return a[0] == b[0];
    }
    let mut c = b.clone();
    c.reverse();
    let d = b.clone();
    let sz = a.len();
    for b in [d, c].iter() {
        let mut ma: HashMap<char, i32> = HashMap::new();
        let mut non_zero = 0;
        let mut splitter = None;
        // println!("check {:?} {:?}", a, b);
        for i in 0..sz {
            let mut now = *ma.get(&a[i]).unwrap_or(&0) as i32;
            now += 1;
            if now == 0 {
                non_zero -= 1;
            }
            if now == 1 {
                non_zero += 1;
            }
            ma.insert(a[i], now);
            now = *ma.get(&b[i]).unwrap_or(&0);
            now -= 1;
            if now == 0 {
                non_zero -= 1;
            }
            if now == -1 {
                non_zero += 1;
            }
            ma.insert(b[i], now);
            // println!("i={} {:?} non_zero={}", i, ma, non_zero);
            if non_zero == 0 {
                splitter = Some(i);
                break;
            }
        }
        if splitter.is_none() {
            continue;
        }
        let splitter = splitter.unwrap();
        if splitter == sz - 1 {
            continue;
        }
        let aa = a[0..=splitter].to_vec();
        let bb = b[0..=splitter].to_vec();
        let aaa = a[splitter + 1..a.len()].to_vec();
        let bbb = b[splitter + 1..a.len()].to_vec();
        let left = go(aa, bb);
        let right = go(aaa, bbb);
        if left && right {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();
        go(a, b)
    }
}

fn main() {
    // println!("{}", Solution::is_scramble("gre".to_string(), "erg".to_string()));
    // println!("{}", Solution::is_scramble("great".to_string(), "rgeat".to_string()));
    // println!("{}", Solution::is_scramble("abcde".to_string(), "caebd".to_string()));
    println!("{}", Solution::is_scramble("unuzp".to_string(), "nzuup".to_string()));
}