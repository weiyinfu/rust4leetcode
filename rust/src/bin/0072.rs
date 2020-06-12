use std::collections::HashMap;
use std::cmp::min;

struct Solution;

fn go(ma: &mut HashMap<(usize, usize), i32>, a: &Vec<char>, b: &Vec<char>, i: usize, j: usize) -> i32 {
    let k = (i, j);
    if ma.contains_key(&k) {
        return *ma.get(&k).unwrap();
    }
    let mut ans = 0i32;
    if i == a.len() || j == b.len() {
        ans = (a.len() - i) as i32 - (b.len() - j) as i32;
        if ans < 0 {
            ans = -ans;
        }
    } else {
        if a[i] == b[j] {
            //如果正好匹配，必定直接匹配
            ans = go(ma, a, b, i + 1, j + 1);
        } else {
            //remove
            let remove1 = go(ma, a, b, i + 1, j);
            let remove2 = go(ma, a, b, i, j + 1);
            //add
            let add1 = go(ma, a, b, i + 1, j);
            let add2 = go(ma, a, b, i, j + 1);
            //replace
            let replace = go(ma, a, b, i + 1, j + 1);
            let op = vec![remove1, remove2, add1, add2, replace];
            let mut mi = op[0];
            for i in op {
                mi = min(mi, i);
            }
            ans = mi + 1;
        }
    }
    ma.insert(k, ans);
    ans
}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let a: Vec<char> = word1.chars().collect();
        let b: Vec<char> = word2.chars().collect();
        //add,remove,replace
        let mut ma = HashMap::<(usize, usize), i32>::new();
        let ans = go(&mut ma, &a, &b, 0, 0);
        // println!("{:?}", ma);
        ans
    }
}


fn main() {
    let ans = Solution::min_distance("horse".to_string(), "ros".to_string());
    println!("{}", ans);
}