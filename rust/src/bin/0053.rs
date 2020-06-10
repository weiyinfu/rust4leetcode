struct Solution;

use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = -(1i64 << 32);
        let mut now = 0i64;
        for i in 0..nums.len() {
            if now > 0 {
                now += nums[i] as i64;
            } else {
                now = nums[i] as i64;
            }
            ans = max(ans, now);
        }
        ans as i32
    }
}

fn main() {
    let ans = Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
    println!("{}", ans);
}