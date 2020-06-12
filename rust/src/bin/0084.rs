use std::cmp::max;

struct Solution;

impl Solution {
    pub fn largest_rectangle_area(mut heights: Vec<i32>) -> i32 {
        if heights.len() == 0 {
            return 0;
        }
        let mut left = vec!(0; heights.len());
        let mut right = vec!(0; heights.len());
        let mut sta = vec!();
        for i in 0..heights.len() {
            while !sta.is_empty() && heights[*sta.last().unwrap()] >= heights[i] {
                sta.pop();
            }
            let which = if sta.is_empty() { 0 } else { *sta.last().unwrap() + 1 };
            left[i] = which;
            sta.push(i);
        }
        sta.clear();
        let mut i = heights.len() - 1;
        loop {
            while !sta.is_empty() && heights[*sta.last().unwrap()] >= heights[i] {
                sta.pop();
            }
            let which = if sta.is_empty() { heights.len() - 1 } else { *sta.last().unwrap() - 1 };
            right[i] = which;
            sta.push(i);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        println!("left = {:?}", left);
        println!("right={:?}", right);
        let mut ans = 0;
        for i in 0..heights.len() {
            let w = right[i] - left[i] + 1;
            let h = heights[i] as usize;
            ans = max(ans, w * h);
        }
        ans as i32
    }
}

fn main() {
    let ans = Solution::largest_rectangle_area(vec!(1,1));
    println!("{}", ans);
}