struct Solution;

fn go(ans: &mut Vec<Vec<i32>>, ind: usize, a: &mut Vec<i32>) {
    if ind == a.len() {
        ans.push(a.clone());
        return;
    }
    for i in ind..a.len() {
        a.swap(ind, i);
        go(ans, ind + 1, a);
        a.swap(ind, i);
    }
}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        go(&mut ans, 0, &mut nums);
        ans
    }
}

fn main() {
    let ans = Solution::permute(vec!(1, 2, 3));
    println!("{:?}", ans);
}