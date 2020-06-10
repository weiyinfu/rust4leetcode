struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let rows = obstacle_grid.len();
        let cols = obstacle_grid[0].len();
        let mut a = vec![vec!(0; cols); rows];
        a[0][0] = 1 - obstacle_grid[0][0];
        for x in 0..rows {
            for y in 0..cols {
                if x == 0 && y == 0 {
                    continue;
                }
                if obstacle_grid[x][y] == 1 {
                    continue;
                }
                let up = if x > 0 { a[x - 1][y] } else { 0 };
                let left = if y > 0 { a[x][y - 1] } else { 0 };
                a[x][y] = up + left;
            }
        }
        a[rows - 1][cols - 1]
    }
}

fn main() {
    let a = vec![
        vec![0, 0, 0, 0],
        vec![0, 1, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0]];
    let ans = Solution::unique_paths_with_obstacles(a);
    println!("{}", ans);
}