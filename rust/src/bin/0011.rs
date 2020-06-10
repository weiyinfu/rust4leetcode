use std::cmp::{min, max};

struct Solution;

fn upper_bound(a: &Vec<usize>, height: &Vec<i32>, v: i32) -> i32 {
    //寻找v在a中的上界
    for i in 0..a.len() {
        if height[a[i] as usize] >= v {
            return a[i] as i32;
        }
    }
    -1
}


fn upper_bound_fast(a: &Vec<usize>, height: &Vec<i32>, v: i32) -> i32 {
    //寻找v在a中的上界，使用二分法
    let mut l = 0usize;
    let mut r = a.len();
    while l < r {
        let mid = (l + r) >> 1;
        if height[a[mid]] < v {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    if l >= a.len() {
        -1
    } else {
        a[l as usize] as i32
    }
}

fn solve(height: &Vec<i32>) -> i32 {
    //左高右低
    let mut s = 0;
    let mut q = Vec::<usize>::new();//上升序列
    for i in 0..height.len() {
        let which = upper_bound_fast(&q, &height, height[i]);
        if which != -1 {
            let area = (i - which as usize) * height[i] as usize;
            s = max(area, s);
        }
        if q.is_empty() || height[*(q.last().unwrap())] < height[i] {
            q.push(i);
        }
    }
    s as i32
}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut a = height.clone();
        let x = solve(&a);
        a.reverse();
        let y = solve(&a);
        return max(x, y);
        0
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}