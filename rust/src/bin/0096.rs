struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }
        let mut s = 1i64;
        let n = n as i64;
        for i in 1..=n {
            s = s * (2 * n + 1 - i) / i;
        }
        (s / (n + 1)) as i32
    }
}

fn main() {
    let ans = Solution::num_trees(19);
    println!("{}", ans);
}