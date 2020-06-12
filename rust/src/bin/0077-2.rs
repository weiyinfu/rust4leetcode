struct Solution;

fn inc(a: &mut Vec<i32>, n: i32) -> bool {
    let mut i = (a.len() - 1) as i32;
    while i >= 0 && a[i as usize] == n - (a.len() as i32 - i) {
        i -= 1;
    }
    if i == -1 {
        return false;
    }
    a[i as usize] += 1;
    for j in i + 1..a.len() as i32 {
        a[j as usize] = a[i as usize] + (j - i);
        if a[j as usize] >= n {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        if k > n {
            return vec!();
        }
        let mut ans: Vec<Vec<i32>> = vec!();
        let mut a = vec!();
        for i in 0..k {
            a.push(i);
        }
        loop {
            let mut now = vec!();
            for i in &a {
                now.push(i + 1);
            }
            ans.push(now);
            let done = inc(&mut a, n);
            if !done {
                break;
            }
        }
        ans
    }
}

fn main() {
    let ans = Solution::combine(3, 2);
    println!("{:?}", ans);
}