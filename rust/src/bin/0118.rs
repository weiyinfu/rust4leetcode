struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec!();
        }
        let mut a = vec!();
        a.push(vec!(1));
        for i in 1..num_rows {
            let mut now = vec!(1);
            let last = a.last().unwrap();
            for j in 1..=i as usize {
                now.push(last[j - 1] + if j >= last.len() { 0 } else { last[j] });
            }
            a.push(now);
        }
        a
    }
}

fn main() {
    println!("{:?}", Solution::generate(5));
}