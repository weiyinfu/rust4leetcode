struct Solution;

use std::iter::FromIterator;

//此题可以用烤馍片算法进行优化
fn starts_with(s: &Vec<char>, i: usize, x: &Vec<char>) -> bool {
    for j in 0..x.len() {
        if i + j >= s.len() {
            return false;
        }
        if x[j] != '?' && s[i + j] != x[j] {
            return false;
        }
    }
    true
}

fn find(s: &Vec<char>, i: usize, x: &Vec<char>) -> i32 {
    //在s中查找字符串x的下标
    for j in i..s.len() {
        if starts_with(s, j, x) {
            return j as i32;
        }
    }
    -1
}

fn solve(s: &Vec<char>, p: &Vec<Vec<char>>) -> bool {
    // println!("{:?}", p);
    // println!("{:?}", s);
    let mut i = 0;
    for x in p {
        // println!("find {:?} in {:?} start {}", x, s, i);
        i = find(s, i as usize, x);
        if i == -1 {
            return false;
        }
        i += x.len() as i32;
    }
    true
}

impl Solution {
    pub fn is_match(mut s: String, mut p: String) -> bool {
        s.insert(0, '$');
        s.push('$');
        p.insert(0, '$');
        p.push('$');
        //把多个*压缩成一个
        let a: Vec<&str> = p.split('*').collect();
        // println!("{:?}", a);
        let mut ss: Vec<char> = s.chars().collect();
        let mut pp: Vec<Vec<char>> = vec!();
        for j in a {
            if j.len() == 0 {
                continue;
            }
            let jj: Vec<char> = j.chars().collect();
            let mut beg = 0;
            let mut end = jj.len() - 1;
            // while beg <= end {
            //     if jj[beg] == '?' {
            //         beg += 1;
            //         continue;
            //     }
            //     break;
            // }
            if beg > end {
                continue;//忽略这个pattern
            }
            pp.push(String::from_iter(jj[beg..=end].to_vec()).chars().collect());
        }
        solve(&ss, &pp)
    }
}

fn main() {
    let ans = Solution::is_match(
        "aaaa".to_string(),
        "***a".to_string());
    println!("{}", ans);
}