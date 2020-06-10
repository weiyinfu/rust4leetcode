use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut a = vec![vec![0; cols]; rows];
        a[0][0] = grid[0][0];
        for i in 0..rows {
            for j in 0..cols {
                if i == 0 && j == 0 {
                    continue;
                }
                let up = grid[i][j] + if i > 0 { a[i - 1][j] } else { 1 << 30 };
                let left = grid[i][j] + if j > 0 { a[i][j - 1] } else { 1 << 30 };
                a[i][j] = min(up, left);
            }
        }
        a[rows - 1][cols - 1]
    }
}

fn main() {
    let a = vec![
        vec![1, 3, 1],
        vec![1, 5, 1],
        vec![4, 2, 1]
    ];
    println!("{:?}", a);
    println!("{}", Solution::min_path_sum(a));
}