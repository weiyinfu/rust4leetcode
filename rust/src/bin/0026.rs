impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut i = 1;
        let mut j = 0;
        while i < nums.len() {
            if nums[i] == nums[j] {} else {
                j += 1;
                nums[j] = nums[i];
            }
            i += 1;
        }
        (j + 1) as i32
    }
}