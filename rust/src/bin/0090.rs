struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec!();
        for i in 0..1 << nums.len() {
            let mut now = vec!();
            for j in 0..nums.len() {
                if i & (1 << j) > 0 {
                    now.push(nums[j]);
                }
            }
            now.sort();
            ans.push(now);
        }
        ans.sort();
        let mut valid = vec!(ans[0].clone());
        for i in 1..ans.len() {
            if ans[i] != ans[i - 1] {
                valid.push(ans[i].clone());
            }
        }
        valid
    }
}

fn main() {
    let ans = Solution::subsets_with_dup(vec!(1, 1, 2, 2, 3));
    println!("{:?}", ans);
}