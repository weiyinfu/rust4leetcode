use std::collections::HashMap;

struct Solution;

/**
这道题可以用特殊的AC技巧通过
*/
fn judge(s: &String, words: &Vec<Vec<char>>, ind: usize, map: &HashMap<&String, i32>) -> bool {
    let mut ma = map.clone();
    let l = words[0].len();
    for gid in 0..words.len() {
        let now = &s[ind + l * gid..ind + l * (gid + 1)];
        let now = now.to_string();
        let v = ma.get_mut(&now);
        if v.is_none() {
            println!("cannot find {}", now);
            return false;
        }
        let value = v.unwrap();
        if *value <= 0 {
            return false;
        }
        *value -= 1;
    }
    true
}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() == 0 || s.len() == 0 {
            return vec!();
        }
        let ss: Vec<char> = s.chars().collect();
        let mut ww = Vec::<Vec<char>>::new();
        for w in &words {
            ww.push(w.chars().collect());
        }
        ww.sort();
        let mut sum = 0i64;
        let mut xorsum = 0i64;
        for w in &ww {
            for ch in w {
                sum += *ch as i64;
                xorsum ^= *ch as i64;
            }
        }
        let mut prefix = Vec::<i64>::new();
        let mut xorprefix = Vec::<i64>::new();
        for i in 0..ss.len() {
            let ch = ss[i] as i64;
            prefix.push(if i == 0 { 0 } else { prefix[i - 1] } + ch);
            xorprefix.push(if i == 0 { 0 } else { xorprefix[i - 1] } ^ ch);
        }
        //构建map
        let mut ma = HashMap::new();
        for w in &words {
            ma.insert(w, ma.get(w).unwrap_or(&0) + 1);
        }

        let len = words[0].len() * words.len();//目标字符串的总长度
        let mut ans = Vec::<i32>::new();
        if ss.len() < len {
            return vec!();
        }
        for i in 0..=ss.len() - len {
            //枚举每一个子串
            let beg_prefix = if i == 0 { 0 } else { prefix[i - 1] };
            let beg_xor_prefix = if i == 0 { 0 } else { xorprefix[i - 1] };
            let cur_xor = xorprefix[i + len - 1] ^ beg_xor_prefix;
            let cur_sum = prefix[i + len - 1] - beg_prefix;
            if cur_xor == xorsum && cur_sum == sum {
                if judge(&s, &ww, i, &ma) {
                    ans.push(i as i32);
                }
            }
        }
        ans
    }
}

fn main() {
    let s = "ba".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let ans = Solution::find_substring(s, words);
    println!("{:?}", ans);
}