
struct Solution;

use std::cmp::min;
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut ans = vec!();
        for k in 0..nums.len() {
            ans.push(nums.len() as i32);
        }
        let mut i = 0;
        ans[0] = 0;
        while i < nums.len() {
            for j in i + 1..=i + nums[i] as usize {
                if j >= ans.len() {
                    break;
                }
                ans[j] = min(ans[j], ans[i] + 1);
            }
            i += 1;
        }
        // println!("{:?}", ans);
        ans[nums.len() - 1]
    }
}


fn main() {
    let ans = Solution::jump(vec!(2, 3, 1, 1, 4));
    println!("{}", ans);
}