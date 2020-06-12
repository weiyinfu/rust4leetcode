struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 { return 1; }
        let mut l = 1i64;
        let x = x as i64;
        let mut r = x >> 1;
        while l + 1 < r {
            let mid = (l + r) >> 1;
            if mid * mid > x {
                r = mid - 1;
            } else {
                l = mid;
            }
        }
        (if r * r > x { l } else { r }) as i32
    }
}

fn main() {
    let ans = Solution::my_sqrt(4);
    println!("{}", ans);
}