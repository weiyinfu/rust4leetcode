struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut cnt = 1;
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[j] {
                cnt += 1;
                if cnt >= 3 {
                    continue;
                } else {
                    j += 1;
                    nums[j] = nums[i];
                }
            } else {
                cnt = 1;
                j += 1;
                nums[j] = nums[i];
            }
        }
        (j + 1) as i32
    }
}

fn main() {
    let mut a = vec!(0, 0, 1, 1, 1, 1, 2, 2);
    let ind = Solution::remove_duplicates(&mut a);
    println!("{} {:?}", ind, a);
}
