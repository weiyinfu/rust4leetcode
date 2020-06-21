struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bits = vec!(0; 32);
        for i in nums {
            for j in 0..32 {
                if (i & (1 << j)) != 0 {
                    bits[j] += 1;
                    bits[j] %= 3;
                }
            }
        }
        let mut x = 0i64;
        for j in 0..32 {
            if bits[j] > 0 {
                x |= (1 << j) as i64;
            }
        }
        x as i32
    }
}

fn main() {
    let ans = Solution::single_number(vec![-2, -2, -2, 1, 1, -3, 1, -3, -3, -2]);
    println!("{}", ans);
}