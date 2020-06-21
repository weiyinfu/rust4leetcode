impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        for i in nums {
            s ^= i;
        }
        s
    }
}