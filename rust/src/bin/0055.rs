
struct Solution;

use std::cmp::max;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }
        let mut x = 0;
        for i in 0..nums.len() {
            if i > x {
                break;
            }
            x = max(x, i + nums[i] as usize);
        }
        x >= nums.len() - 1
    }
}

fn main() {
    let x = Solution::can_jump(vec![2, 3, 1, 1, 4]);
    let y = Solution::can_jump(vec![3, 2, 1, 0, 4]);
    println!("{} {}", x, y);
}