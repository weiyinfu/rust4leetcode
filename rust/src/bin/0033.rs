struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut x = 0;
        let mut step = 1;
        while step <= nums.len() {
            step <<= 1;
        }
        step >>= 1;
        while step > 0 {
            let ind = x + step;
            if ind < nums.len() && nums[ind] > nums[0] {
                //找到了
                x += step;
            }
            step >>= 1;
        }
        x += 1;
        let mut ans;
        if target >= nums[0] {
            //在前半部分寻找
            ans = nums[..x].binary_search(&target);
            if ans.is_err() {
                return -1;
            } else {
                return ans.unwrap() as i32;
            }
        } else {
            ans = nums[x..].binary_search(&target);
            if ans.is_err() {
                return -1;
            }
            return (ans.unwrap() + x) as i32;
        }
    }
}


fn main() {
    let ans = Solution::search(vec![4], 0);
    println!("{}", ans);
}