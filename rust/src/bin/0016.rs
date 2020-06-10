struct Solution;

fn search(a: &Vec<i32>, beg: i32, end: i32, value: i32) -> i32 {
    //二分搜索下确界
    let mut l = beg;
    let mut r = end;
    while l + 1 < r {
        let mid = (l + r) >> 1;
        if a[mid as usize] < value {
            l = mid;
        } else {
            r = mid;
        }
    }
    l
}

fn abs(x: i32) -> i32 {
    if x < 0 { -x } else { x }
}

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 1 << 30;
        let mut a = nums;
        a.sort();
        let mut check = |now: i32| {
            if abs(now - target) < abs(ans - target) {
                ans = now;
            }
        };
        for i in 0..a.len() {
            for j in i + 1..a.len() {
                let need = target - a[i] - a[j];
                let ind = search(&a, (j + 1) as i32, (a.len() - 1) as i32, need);
                //需要同时检查上界和下界
                if ind < a.len() as i32 {
                    check(a[i] + a[j] + a[ind as usize])
                }
                if ind + 1 < a.len() as i32 {
                    check(a[i] + a[j] + a[(ind + 1) as usize]);
                }
            }
        }
        ans
    }
}

fn main() {
    let ans = Solution::three_sum_closest(vec![-4, -7, -2, 2, 5, -2, 1, 9, 3, 9, 4, 9, -9, -3, 7, 4, 1, 0, 8, 5, -7, -7], 29);
    println!("{}", ans);
}