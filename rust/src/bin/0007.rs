struct Solution;

const MAX: i64 = (1i64 << 31) - 1;
const MIN: i64 = -(1i64 << 31);

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut a = Vec::<i64>::new();
        let xx = x as i64;
        let flag = if xx > 0 { 1 } else { -1 };
        let mut i = if xx > 0 { xx } else { -xx } as i64;
        while i > 0 {
            a.push(i % 10);
            i /= 10;
        }
        let mut y = 0i64;
        for i in 0..a.len() {
            y = y * 10 + a[i];
        }
        y *= flag;
        if y > MAX || y < MIN {
            return 0;
        }
        return y as i32;
    }
}

fn main() {
    // println!("{}", Solution::reverse(1534236469));
    // println!("{}", -(1i64 << 31));
    // println!("{}", Solution::reverse(-(1i64 << 31) as i32));
    println!("{} {} {}", Solution::reverse(1463847412), MIN, MAX);
}