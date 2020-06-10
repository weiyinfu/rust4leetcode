struct Solution;

fn upper(a: &Vec<i32>, x: i32) -> i32 {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l + 1 < r {
        let mid = (l + r) >> 1;
        if a[mid] > x {
            r = mid - 1;
        } else {
            l = mid;
        }
    }
    if r >= 0 && r < a.len() && a[r] == x {
        r as i32
    } else {
        if l < a.len() && a[l] == x {
            return l as i32;
        }
        -1
    }
}

fn floor(a: &Vec<i32>, x: i32) -> i32 {
    let mut l = 0;
    let mut r = a.len() - 1;
    while l < r {
        let mid = (l + r) >> 1;
        if a[mid] < x {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    if l >= 0 && l < a.len() && a[l] == x {
        l as i32
    } else {
        if r >= 0 && r < a.len() && a[r] == x {
            return r as i32;
        }
        -1
    }
}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec!(-1, -1);
        }
        return vec!(floor(&nums, target), upper(&nums, target));
    }
}

fn main() {
    let nums = vec![2, 2];
    let target = 0;
    let ans = Solution::search_range(nums, target);
    println!("{:?}", ans);
}