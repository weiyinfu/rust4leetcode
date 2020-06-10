use std::cmp::min;
use std::ops::Add;

//最长回文子串，马拉车算法
struct Solution;

fn odd_count(beg: i32, end: i32) -> i32 {
    //计算一个区间中奇数的个数
    let mut b = beg;
    let mut e = end;
    if (beg & 1) != 0 {
        b -= 1;
    }
    if (e & 1) != 0 {
        e += 1;
    }
    return (e - b) / 2;
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //预处理，一切回文一定是奇数个字符串，为了防止溢出，首尾添加哨兵单元（特殊字符）
        let mut ss = Vec::<char>::new();
        let mut is_first = true;
        for i in s.chars() {
            //#a#a#b#a#b#
            if is_first {
                ss.push('^');
                is_first = false;
            } else {
                ss.push('#');
            }
            ss.push(i);
        }
        ss.push('$');
        println!("预处理之后的字符串{:?}", ss);
        let mut right = 0i32;//右侧指针到达的最远距离
        let mut center = 0i32;//能够到达最远右侧的中心

        //每个中心处的最长回文子串
        let mut le = Vec::<i32>::with_capacity(ss.len());
        for i in 0..ss.len() {
            le.push(0);
        }
        for i in 1..ss.len() as i32 {
            // println!("i={} center={} right={}", i, center, right);
            let mut j;//右边应该从何处开始迭代
            if i >= right {
                //需要当前结点去开辟新空间
                center = i;
                right = i;
                j = 0;
            } else {
                //利用过去已经探索过的内容
                let brother = center * 2 - i;
                let brother_len = le[brother as usize];
                let to_right = right - i;
                if to_right > brother_len {
                    //如果到右侧的距离大于brother_len，可以直接更改
                    le[i as usize] = le[brother as usize];
                    continue;
                } else {
                    j = to_right;
                }
            }
            while i - j >= 0
                && i + j < ss.len() as i32
                && ss[(i - j) as usize] == ss[(i + j) as usize] {
                j += 1;
            }
            j -= 1;
            // println!("{}处的回文长度为{}", i, j);
            le[i as usize] = j;//以i为中心最长可达到的最大长度
            center = i;//新的中心，能够到达这一步说明i一定是开辟了新疆域
            right = i + j;
            // println!("len({})={}", i, j);
        }
        let mut ans = (0, 0);
        let mut max_len = 0;
        for i in 0..ss.len() as i32 {
            //计算有效字符的长度
            let mut len = odd_count(i - le[i as usize], i + le[i as usize]);
            // println!("{}={} {}", i, le[i as usize], len);
            if len > max_len {
                max_len = len;
                ans = (i - le[i as usize], i + le[i as usize]);
            }
        }
        let mut ans_string = "".to_string();
        for i in ans.0..=ans.1 {
            if i % 2 == 0 {
                continue;
            }
            ans_string.push(ss[i as usize]);
        }
        return ans_string;
    }
}

fn main() {
    let res = Solution::longest_palindrome("babad".to_string());
    println!("{}", res);
}