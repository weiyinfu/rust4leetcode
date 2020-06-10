struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut x = dividend as i64;
        let mut y = divisor as i64;
        //判断溢出
        if x == -(1i64 << 31) && y == -1 {
            return ((1i64 << 31) - 1) as i32;
        }
        let mut flag: i32 = ((x < 0) as i32) ^ ((y < 0) as i32);
        if x < 0 { x = -x }
        if y < 0 { y = -y }
        let mut cnt = 0;
        while y <= x {
            y <<= 1;
            cnt += 1;
        }
        let mut ans = 0;
        while cnt >= 0 {
            if x >= y {
                x -= y;
                ans |= (1 << cnt);
            } else {
                y >>= 1;
                cnt -= 1;
            }
        }
        if flag == 1 {
            ans *= -1;
        }
        ans
    }
}

fn main() {
    let ans = Solution::divide(2147483647, 1);
    println!("{}", ans);
}