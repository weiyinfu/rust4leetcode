use std::collections::HashMap;

struct Solution;

fn check(m: &Vec<(usize, usize)>, n: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut ans = Vec::<Vec<usize>>::new();
    for i in m {
        for j in n {
            let mut four = vec!(i.0, i.1, j.0, j.1);
            four.sort();
            four.dedup_by_key(|x| *x);
            if four.len() == 4 {
                ans.push(four);
            }
        }
    }
    ans
}

fn map(index: &Vec<Vec<usize>>, a: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = Vec::<Vec<i32>>::new();
    for i in index {
        let mut now = Vec::<i32>::new();
        for j in i {
            now.push(a[*j]);
        }
        now.sort();
        ans.push(now);
    }
    ans
}

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut a = HashMap::<i32, Vec<(usize, usize)>>::new();
        let mut x = nums.to_owned();
        x.sort();
        for i in 0..x.len() {
            for j in i + 1..x.len() {
                let s = x[i] + x[j];
                if !a.contains_key(&s) {
                    //如果不包含key
                    a.insert(s, Vec::new());
                }
                let ar = a.get_mut(&s).unwrap();
                ar.push((i, j));
            }
        }
        let mut ks: Vec<i32> = a.keys().map(|x| { *x }).collect();
        ks.sort();
        let mut j = (ks.len() - 1) as isize;
        let mut i = 0;
        let mut good = Vec::<Vec<i32>>::new();
        while i <= j {
            while i <= j && ks[i as usize] + ks[j as usize] < target {
                i += 1;
            }
            while i <= j && ks[i as usize] + ks[j as usize] > target {
                j -= 1;
            }
            if i <= j && ks[i as usize] + ks[j as usize] == target {
                let four = check(&a.get(&ks[i as usize]).unwrap(), &a.get(&ks[j as usize]).unwrap());
                good.extend(map(&four, &x));
            }
            i += 1;
        }
        good.sort();
        good.dedup_by_key(|x| { (x[0], x[1], x[2], x[3]) });
        good
    }
}

fn main() {
    let ans = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    println!("{:?}", ans);
}