struct Solution;


fn go(ans: &mut Vec<Vec<i32>>, ind: usize, a: &mut Vec<i32>) {
    // println!("ind={},a={:?}", ind, a);
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
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        go(&mut ans, 0, &mut nums);
        ans.sort();
        if ans.len() == 0 {
            return vec![];
        }
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
    let ans = Solution::permute_unique(vec!(1, 1, 2));
    println!("{:?}", ans);
}