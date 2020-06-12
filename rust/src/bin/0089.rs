struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = vec!();
        for i in 0..(1 << n) {
            ans.push(i ^ (i >> 1));
        }
        ans
    }
}

fn main() {
    let ans = Solution::gray_code(3);
    println!("{:?}", ans);
}