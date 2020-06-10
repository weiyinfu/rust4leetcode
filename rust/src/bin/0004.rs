struct Solution;

const MAX: i32 = (1 << 30) - 1;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn get(a: &Vec<i32>, ind: i32) -> i32 {
    //使用哨兵单元作为最后元素，这样两个数组都不会遇到结尾
    if ind as usize >= a.len() {
        return MAX;
    } else {
        return a[ind as usize];
    }
}

fn get_nth(a: &Vec<i32>, b: &Vec<i32>, n: i32) -> i32 {
    //寻找第n个元素
    let mut i: i32 = -1;
    let mut j: i32 = -1;
    loop {
        // println!("i={},j={},target={}", i, j, n);
        if i + j + 1 == n {
            //找到了第n个元素
            if j == -1 {
                return a[i as usize];
            }
            if i == -1 {
                return b[j as usize];
            }
            return max(a[i as usize], b[j as usize]);
        }
        let mut dis = (n - i - j - 1) / 2;
        if dis == 0 {
            //最少移动1步，防止剩下2步的时候原地踏步
            dis = 1;
        }
        let x = get(&a, i + dis);
        let y = get(&b, j + dis);
        //两者让小的那一个前进
        if x < y {
            i += dis;
        } else {
            j += dis;
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let l = nums2.len() + nums1.len();
        if l % 2 == 0 {
            let x = get_nth(&nums1, &nums2, (l / 2) as i32);
            let y = get_nth(&nums1, &nums2, (l / 2 - 1) as i32);
            // println!("found even {} {}", x, y);
            return (x + y) as f64 / 2.0;
        } else {
            let x = get_nth(&nums1, &nums2, (l / 2) as i32);
            // println!("found odd {}", x);
            return x as f64;
        }
        unreachable!();
    }
}

fn main() {
    let res = Solution::find_median_sorted_arrays(vec![0, 0], vec!(0, 0));
    println!("{}", res);
}