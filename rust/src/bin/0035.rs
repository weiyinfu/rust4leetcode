impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let a = nums;
        let x = target;
        let mut r = a.len();
        while l < r {
            let mid = (l + r) >> 1;
            if a[mid] < x {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}