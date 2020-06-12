use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximal_rectangle(mut matrix: Vec<Vec<char>>) -> i32 {
        let rows = matrix.len();
        if rows == 0 {
            return 0;
        }
        let cols = matrix[0].len();
        if cols == 0 {
            return 0;
        }
        let mut a = vec!(0; cols);//ai表示第i列的1的上限
        let mut left = vec!(0; cols);
        let mut right = vec!(cols - 1; cols);
        let mut ans = 0;
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    a[j] += 1;
                } else {
                    a[j] = 0;
                }
            }
            let mut last_zero = None;
            for j in 0..cols {
                if a[j] == 0 {
                    last_zero = Some(j);
                    left[j] = 0;
                    right[j] = cols - 1;
                } else {
                    if last_zero.is_some() && left[j] <= last_zero.unwrap() {
                        left[j] = last_zero.unwrap() + 1;
                    }
                }
            }
            let mut last_zero = None;
            let mut j = cols - 1;
            while j >= 0 {
                if a[j] == 0 {
                    last_zero = Some(j);
                } else {
                    if last_zero.is_some() && right[j] >= last_zero.unwrap() {
                        right[j] = last_zero.unwrap() - 1;
                    }
                }
                if j == 0 {
                    break;
                }
                j -= 1;
            }
            // println!("{:?}", left);
            // println!("{:?}", right);
            // println!("{:?}", a);
            // println!("++++++");
            for i in 0..a.len() {
                let w = right[i] - left[i] + 1;
                let h = a[i];
                ans = max(ans, w * h);
            }
        }
        ans as i32
    }
}


impl Solution {
    /**
    会超时的算法，比较暴力，但是可以用于对拍
    */
    pub fn maximal_rectangle2(mut matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 {
            return 0;
        }
        //在第一行、第一列添加0
        matrix.insert(0, vec!['0'; matrix[0].len()]);
        for i in matrix.iter_mut() {
            i.insert(0, '0');
        }
        let rows = matrix.len();
        let cols = matrix[0].len();
        if cols == 0 {
            return 0;
        }
        let mut a = vec!(vec!(0; cols); rows);
        let mut ans = 0;
        for i in 1..rows {
            for j in 1..cols {
                let top = if i > 0 { a[i - 1][j] } else { 0 };
                let left = if j > 0 { a[i][j - 1] } else { 0 };
                let corner = if i > 0 && j > 0 { a[i - 1][j - 1] } else { 0 };
                let area = left + top - corner;
                let ch = if matrix[i][j] == '0' { 0 } else { 1 };
                a[i][j] = area + ch;
                if matrix[i][j] == '1' {
                    for x in 1..=i {
                        for y in 1..=j {
                            if matrix[x][y] == '1' {
                                let now = a[i][j] + a[x - 1][y - 1] - a[i][y - 1] - a[x - 1][j];
                                if now == (i - x + 1) * (j - y + 1) {
                                    if now == 70 {
                                        println!("{:?}", (i, j, x, y));
                                    }
                                    ans = max(now, ans);
                                }
                            }
                        }
                    }
                }
            }
        }
        // println!("{:?}", a);
        ans as i32
    }
}

fn main() {
    let a = "111111111111
011101111111
011111111111
110011111111
011111111111
111111111111
111111111111
111111111111
111111111111
111111111111
111111111101";
    let mut ma = vec!();
    for i in a.split("\n").collect::<Vec<&str>>() {
        let mut row = vec!();
        for j in i.chars() {
            row.push(j);
        }
        ma.push(row);
    }
    for i in &ma {
        for j in i {
            print!("{}", *j);
        }
        println!();
    }
    let ans = Solution::maximal_rectangle(ma.clone());
    println!("{}", ans);
    let ans2 = Solution::maximal_rectangle2(ma);
    println!("{}", ans2);
}