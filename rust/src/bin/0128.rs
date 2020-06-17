use std::collections::HashMap;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ma = HashMap::new();
        let mut ans = 0;
        for i in nums {
            if ma.contains_key(&i) {
                continue;
            }
            let left = *ma.get(&(i - 1)).unwrap_or(&0);
            let right = *ma.get(&(i + 1)).unwrap_or(&0);
            let now = left + right + 1;
            ma.insert(i, now);
            ma.insert(i - left, now);
            ma.insert(i + right, now);
            ans = max(ans, now);
        }
        ans
    }
}

fn main() {
    let ans = Solution::longest_consecutive(vec!(1, 2, 0, 1));
    println!("{}", ans);
}