struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() == 0 {
            return false;
        }
        let last = *nums.last().unwrap();
        let inleft = target > last;
        if target == last {
            return true;
        }
        let mut l = 0;
        while l < nums.len() && nums[l] == last { l += 1; }
        if l == nums.len() {
            return false;
        }
        let mut r = nums.len() - 1;
        while l + 1 < r {
            let mid = (l + r) >> 1;
            println!("l={},r={},mid={}", l, r, mid);
            //中间值
            if inleft {
                if nums[mid] <= last {
                    r = mid - 1;
                } else {
                    if nums[mid] < target {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
            } else {
                if nums[mid] > last {
                    l = mid + 1;
                } else {
                    if nums[mid] < target {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
            }
        }
        nums[l] == target || nums[r] == target
    }
}

fn main() {
    let ans = Solution::search(vec![1, 1, 3, 1], 3);
    println!("{}", ans);
}