struct Solution;

fn search(a: &Vec<i32>, beg: i32, end: i32, value: i32) -> i32 {
    let mut l = beg;
    let mut r = end;
    while l < r {
        let mid = (l + r) >> 1;
        if a[mid as usize] < value {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    if l < a.len() as i32 && a[l as usize] == value {
        l
    } else {
        -1
    }
}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut a = nums.clone();
        a.sort();
        // println!("{:?}", a);
        let mut ans = Vec::<Vec<i32>>::new();
        let mut submit = |it: Vec<i32>| {
            ans.push(it);
        };
        let mut i = 0;
        while i < a.len() {
            let mut j = i + 1;
            while j < a.len() {
                let k = 0 - a[i] - a[j];
                let ind = search(&a, (j + 1) as i32, (a.len() - 1) as i32, k);
                if ind == -1 {
                    //如果没有搜到
                    j += 1;
                    continue;
                }
                // println!("{} {} ", i, j);
                submit(vec!(a[i], a[j], a[ind as usize]));
                let k = a[j];
                j += 1;
                while j < a.len() && a[j] == k {
                    j += 1;
                }
            }
            let k = a[i];
            i += 1;
            while i < a.len() && a[i] == k {
                i += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]))
}