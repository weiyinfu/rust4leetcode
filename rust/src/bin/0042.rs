struct Solution;


use std::cmp::{max, min};


impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut leftMax = Vec::<i32>::with_capacity(height.len());
        let mut rightMax = Vec::<i32>::with_capacity(height.len());
        unsafe { leftMax.set_len(height.len()); }
        unsafe { rightMax.set_len(height.len()); }
        let mut m = 0;
        for i in 0..height.len() {
            leftMax[i] = m;
            m = max(height[i], m);
        }
        m = 0;
        let mut i = (height.len() - 1) as i32;
        while i >= 0 {
            rightMax[i as usize] = m;
            m = max(height[i as usize], m);
            i -= 1;
        }
        let mut s = 0;
        for i in 0..height.len() {
            let h = min(rightMax[i], leftMax[i]);
            if h > height[i] {
                s += h - height[i];
            }
        }
        println!("{:?}", leftMax);
        println!("{:?}", rightMax);
        s
    }
}

fn main() {
    let a = Solution::trap(vec!(0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1));
    println!("{}", a);
}