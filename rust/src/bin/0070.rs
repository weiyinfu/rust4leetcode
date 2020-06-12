struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = vec!();
        a.push(1);
        a.push(1);
        for i in 2..=n as usize {
            let x = a[i - 1] + a[i - 2];
            a.push(x);
        }
        println!("{:?}",a);
        a[n as usize]
    }
}

fn main() {
    let ans = Solution::climb_stairs(5);
    println!("{}", ans);
}