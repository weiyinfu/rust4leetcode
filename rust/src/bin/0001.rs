struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            index.push(i as i32);
        }
        index.sort_by_key(|x| { nums[*x as usize] });
        let mut j = (nums.len() - 1) as i32;
        let mut i = 0;
        while i < j {
            let x = nums[index[i as usize] as usize];
            let y = nums[index[j as usize] as usize];
            if x + y > target {
                j -= 1;
            } else if x + y < target {
                i += 1;
            } else {
                return vec!(index[i as usize], index[j as usize]);
            }
        }
        unreachable!()
    }
}

fn main() {
    let ans = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", ans);
}