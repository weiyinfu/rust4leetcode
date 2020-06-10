struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits.len() == 0 {
            return vec!(1);
        };
        let mut i: i32 = digits.len() as i32 - 1;
        while i >= 0 {
            if digits[i as usize] == 9 {
                digits[i as usize] = 0;
            } else {
                break;
            }
            i -= 1;
        }
        if i == -1 {
            digits.insert(0, 1);
        } else {
            digits[i as usize] += 1;
        }
        digits
    }
}

fn main() {
    let ans = Solution::plus_one(vec!());
    println!("{:?}", ans);
}