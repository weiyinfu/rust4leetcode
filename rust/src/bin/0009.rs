struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut a = Vec::<i32>::new();
        let mut i = x;
        while i > 0 {
            a.push(i % 10);
            i /= 10;
        }
        for i in 0..a.len() / 2 {
            if a[i] != a[a.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(12321));
}