struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnt = [0; 3];
        for i in nums.iter() {
            cnt[*i as usize] += 1;
        }
        let mut j = 0;
        for i in 0..3 {
            for k in 0..cnt[i] {
                nums[j] = i as i32;
                j += 1;
            }
        }
    }
}

fn main() {}