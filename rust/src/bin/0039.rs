struct Solution;

fn go(cand: &Vec<i32>, target: i32, ind: usize, s: i32, ans: &mut Vec<Vec<i32>>, now: &mut Vec<i32>) {
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
    while s + cnt * i <= target {
        go(cand, target, ind + 1, s + cnt * i, ans, now);
        now.push(i);
        cnt += 1;
    }
    while cnt > 0 {
        now.pop();
        cnt -= 1;
    }
}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        candidates.dedup_by_key(|x| { *x });
        let mut ans: Vec<Vec<i32>> = vec!();
        go(&candidates, target, 0, 0, &mut ans, &mut vec!());
        ans
    }
}

fn main() {
    let ans = Solution::combination_sum(vec!(2, 3, 6, 7), 7);
    println!("{:?}", ans);
}