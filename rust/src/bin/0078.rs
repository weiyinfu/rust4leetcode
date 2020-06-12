struct Solution;

fn go(ans: &mut Vec<Vec<i32>>, now: &mut Vec<i32>, ind: usize, a: &Vec<i32>) {
    if ind == a.len() {
        ans.push(now.clone());
        return;
    }
    now.push(a[ind]);
    go(ans, now, ind + 1, a);
    now.pop();
    go(ans, now, ind + 1, a);
}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec!();
        let mut now = vec!();
        go(&mut ans, &mut now, 0, &nums);
        ans
    }
}

fn main() {
    let ans = Solution::subsets(vec!(1, 2, 3));
    println!("{:?}", ans);
}