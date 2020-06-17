use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] += triangle[i - 1][0];
                    continue;
                }
                if j == triangle[i].len() - 1 {
                    triangle[i][j] += triangle[i - 1][j - 1];
                    continue;
                }
                triangle[i][j] += min(triangle[i - 1][j], triangle[i - 1][j - 1]);
            }
        }
        let mut mi: i64 = 0xffffffffff;
        for i in triangle.last().unwrap() {
            if mi > *i as i64 {
                mi = *i as i64;
            }
        }
        mi as i32
    }
}


fn main() {
    println!("hello");
    println!("{}",Solution::minimum_total(vec![
        vec![2],
        vec![3, 4],
        vec![6, 5, 7],
        vec![4, 1, 8, 3]
    ]))
}