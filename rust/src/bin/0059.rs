struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut a = vec!();
        let passwd = 0xdfa;
        for i in 0..n {
            let mut row = vec!();
            for j in 0..n {
                row.push(passwd);
            }
            a.push(row);
        }
        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut d = 0;
        let mut x = 0;
        let mut y = 0;
        let legal = |x, y| {
            x >= 0 && y >= 0 && x < n && y < n
        };
        let mut i = 1;
        loop {
            a[x as usize][y as usize] = i;
            i += 1;
            let (mut xx, mut yy) = (x + direction[d].0, y + direction[d].1);
            if !legal(xx, yy) || a[xx as usize][yy as usize] != passwd {
                d = (d + 1) % 4;
                xx = x + direction[d].0;
                yy = y + direction[d].1;
                if !legal(xx, yy) || a[xx as usize][yy as usize] != passwd {
                    break;
                }
            }
            x = xx;
            y = yy;
        }
        a
    }
}

fn main() {
    let ans = Solution::generate_matrix(3);
    println!("{:?}", ans);
}