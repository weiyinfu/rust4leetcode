struct Solution;

use std::collections::HashMap;
use std::ops::DerefMut;

fn go(cand: &Vec<i32>, target: i32, ind: usize, s: i32, ans: &mut Vec<Vec<i32>>,
      now: &mut Vec<i32>,
      ma: &HashMap<i32, i32>) {
    if s > target {
        return;
    }
    if ind == cand.len() {
        if s == target {
            ans.push(now.clone());
        }
        return;
    }
    let i = cand[ind];
    let mut cnt = 0;
    // println!("ind={} i={} s={} now={:?}", ind, i, s, now);
    while s + cnt * i <= target && cnt <= *ma.get(&i).unwrap() {
        go(cand, target, ind + 1, s + cnt * i, ans, now, ma);
        now.push(i);
        cnt += 1;
    }
    while cnt > 0 {
        now.pop();
        cnt -= 1;
    }
}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut ma = HashMap::new();
        for i in &candidates {
            let mut v = ma.get_mut(i);
            if v.is_none() {
                ma.insert(*i, 1);
            } else {
                let vv = v.unwrap();
                *vv += 1;
            }
        }
        candidates.dedup_by_key(|x| *x);
        println!("{:?}", ma);
        let mut ans: Vec<Vec<i32>> = vec!();
        go(&candidates, target, 0, 0, &mut ans, &mut vec!(), &ma);
        ans
    }
}

fn main() {
    let ans = Solution::combination_sum2(vec!(10, 1, 2, 7, 6, 1, 5), 8);
    println!("{:?}", ans);
}