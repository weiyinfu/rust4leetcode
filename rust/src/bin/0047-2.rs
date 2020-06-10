struct Solution;
use std::collections::HashSet;



fn go(ans: &mut Vec<Vec<i32>>, ind: usize, a: &mut Vec<i32>) {
    // println!("ind={},a={:?}", ind, a);
    if ind == a.len() {
        ans.push(a.clone());
        return;
    }
    let mut used = HashSet::new();
    for i in ind..a.len() {
        if used.contains(&a[i]) {
            continue;
        }
        used.insert(a[i]);
        a.swap(ind, i);
        go(ans, ind + 1, a);
        a.swap(ind, i);
    }
}

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        go(&mut ans, 0, &mut nums);
        ans
    }
}


fn main() {
    let ans = Solution::permute_unique(vec!(1, 1, 2));
    println!("{:?}", ans);
}