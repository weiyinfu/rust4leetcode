struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in m..m + n {
            nums1[i as usize] = nums2[(i - m) as usize];
        }
        nums1.sort();
    }
}

fn main() {
    let mut a = vec!(-1, 0, 0, 1);
    let mut b = vec!(-1, -1, 0, 2);
    let m = a.len();
    let n = b.len();
    a.append(&mut vec!(0; b.len()));
    Solution::merge(&mut a, m as i32, &mut b, n as i32);
    println!("{:?}", a);
    println!("{:?}", b);
}