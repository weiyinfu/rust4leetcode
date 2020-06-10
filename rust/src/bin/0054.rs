struct Solution;

impl Solution {
    pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return vec!();
        }
        let mut x = 0i32;
        let mut y = 0i32;
        let mut ans = vec![];
        let direction = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut now = 0;
        let passwd = 0x34343;
        let mut bad = 0;
        loop {
            if matrix[x as usize][y as usize] != passwd {
                ans.push(matrix[x as usize][y as usize]);
                matrix[x as usize][y as usize] = passwd;
            }
            let (xx, yy) = (x + direction[now].0, y + direction[now].1);
            if !(xx >= 0 && xx < matrix.len() as i32 && yy >= 0 && yy < matrix[0].len() as i32)
                || matrix[xx as usize][yy as usize] == passwd {
                now += 1;
                now %= 4;
                bad += 1;
                if bad > 5 {
                    break;
                }
            } else {
                bad = 0;
                x = xx;
                y = yy;
            }
            // println!("x={} y={}", x, y);
        }
        ans
    }
}

fn main() {
    let a = vec![
        vec![1]
    ];
    let ans = Solution::spiral_order(a);
    println!("{:?}", ans);
}