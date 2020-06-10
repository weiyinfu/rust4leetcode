struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, mut k: i32) -> String {
        let mut a = vec!();
        for i in 0..n {
            a.push(true);
        }
        let mut ans: Vec<i32> = vec!();
        let mut fac = [0; 10];
        fac[0] = 1;
        for i in 1..fac.len() {
            fac[i] = fac[i - 1] * i;
        }
        k -= 1;
        for i in 0..n {
            //是第几个未被使用的数字
            let nth = k / fac[(n - i - 1) as usize] as i32 + 1;
            k %= fac[(n - i - 1) as usize] as i32;
            let mut which = 0;
            let mut good = 0;
            while which < n {
                if a[which as usize] {
                    good += 1;
                    if good == nth {
                        break;
                    }
                }
                which += 1;
            }
            if which == n {
                break;
            }
            a[which as usize] = false;
            ans.push(which);
        }
        let mut s = String::new();
        for i in ans {
            let ss: String = (i + 1).to_string();
            s.push_str(&ss);
        }
        s
    }
}

fn main() {
    for i in 1..=6 {
        let ans = Solution::get_permutation(3, i);
        println!("{} {}", i, ans);
    }
}