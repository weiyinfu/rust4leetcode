struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            while nums[i] <= nums.len() as i32
                && nums[i] > 0
                && nums[i] != (i + 1) as i32 {
                //如果数字在合理范围内，把它放到合适位置
                let p = nums[i] - 1;
                //p必定是一个合理的值
                if nums[i] == nums[p as usize] {
                    //如果相等，直接跳出
                    break;
                }
                nums.swap(i, p as usize);
            }
        }
        for i in 0..nums.len() {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}


fn main() {
    let x = Solution::first_missing_positive(vec!(1, 1));
    println!("{}", x);
}