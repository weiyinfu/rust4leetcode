struct Solution;

fn go(x: f64, n: i64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let son = go(x, n / 2);
    let mut now = son * son;
    if n % 2 == 1 {
        now *= x;
    }
    now
}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let nn = n as i64;
        if n < 0 {
            1.0 / go(x, -nn)
        } else {
            go(x, nn)
        }
    }
}

fn main() {
    let x = Solution::my_pow(2.0, -2);
    println!("{}", x);
}