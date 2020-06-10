
struct Solution;

use std::cmp::min;
impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        m -= 1;
        n -= 1;
        let c = m + n;
        if c == 0 {
            return 1;
        }
        let mut ans = 1i64;
        for i in 1..=min(m, n) {
            ans = ans * (c + 1 - i) as i64 / i as i64;
        }
        ans as i32
    }
}

fn main() {
    let a = Solution::unique_paths(3, 2);
    println!("{}", a);
}