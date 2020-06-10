struct Solution;

use std::mem::swap;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() == 0 {
            return;
        }
        let mut i = nums.len() - 1;
        while i > 0 && nums[i] <= nums[i - 1] {
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
            return;
        }
        let target = nums[i - 1];
        let mut l = nums.len() - 1;
        while l > i && nums[l] <= target {
            l -= 1;
        }
        nums.swap(l, i - 1);
        let mut j = nums.len() - 1;
        while j > i {
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}


fn main() {
    let mut a = vec!(1, 1, 5);
    for i in 0..7 {
        println!("{:?}", a);
        Solution::next_permutation(&mut a);
    }
    println!("{:?}", a);
}