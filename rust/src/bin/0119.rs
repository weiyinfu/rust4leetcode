struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec!(1);
        }
        let mut a = vec!(1);
        for i in 1..=row_index as usize {
            let now = a[i - 1] as i64 * (row_index as i64 + 1 - i as i64) / i as i64;
            a.push(now as i32);
        }
        a
    }
}

fn main() {
    println!("{:?}", Solution::get_row(4));
}