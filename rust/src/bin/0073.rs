struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let passwd = 0xfacd;
        let rows = matrix.len();
        let cols = matrix[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == 0 {
                    println!("{} {}", i, j);
                    for ii in 0..rows {
                        if matrix[ii][j] != 0 {
                            matrix[ii][j] = passwd;
                        }
                    }
                    for jj in 0..cols {
                        if matrix[i][jj] != 0 {
                            matrix[i][jj] = passwd;
                        }
                    }
                }
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == passwd {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut a = vec![
        vec![0, 1, 2, 0],
        vec![3, 4, 5, 2],
        vec![1, 3, 1, 5]
    ];
    Solution::set_zeroes(&mut a);
    println!("{:?}", a);
}