struct Solution;

fn go(ans: &mut Vec<Vec<i32>>, a: &mut Vec<i32>, cur: usize, k: usize, n: usize) {
    if a.len() == k {
        ans.push(a.clone());
        return;
    }
    if cur >= n {
        return;
    }
    a.push((cur + 1) as i32);
    go(ans, a, cur + 1, k, n);
    a.pop();
    go(ans, a, cur + 1, k, n);
}

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec!();
        let mut a = vec!();
        go(&mut ans, &mut a, 0, k as usize, n as usize);
        ans
    }
}

fn main() {
    let ans = Solution::combine(3, 2);
    println!("{:?}", ans);
}