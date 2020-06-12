struct Solution;

fn pack(words: &Vec<String>, beg: usize, end: usize, max_width: i32) -> String {
    let mut sz = 0;
    for i in beg..end {
        sz += words[i].len();
    }
    let cnt = end - beg;
    if cnt == 1 {
        let mut s = words[beg].to_string();
        for i in s.len()..max_width as usize {
            s.push(' ');
        }
        return s;
    }
    let space = (max_width - sz as i32) / (cnt - 1) as i32;
    //第一个空格的大小
    let mut left = max_width - sz as i32 - space * (cnt - 1) as i32;
    let space = " ".repeat(space as usize);
    let mut s = words[beg].to_string();
    for i in beg + 1..end {
        if left > 0 {
            left -= 1;
            s.push(' ');
        }
        s.push_str(space.as_str());
        s.push_str(words[i].as_str());
    }
    s
}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ans = vec!();
        let mut i = 0;
        let mut cur_width = 0;
        for j in 0..words.len() {
            if cur_width != 0 {
                cur_width += 1;
            }
            cur_width += words[j].len();
            if cur_width > max_width as usize {
                let line = pack(&words, i, j, max_width);
                ans.push(line);
                cur_width = words[j].len();
                i = j;
            }
        }
        let mut s = String::new();
        s.push_str(words[i].as_str());
        i += 1;
        while i < words.len() {
            s.push(' ');
            s.push_str(words[i].as_str());
            i += 1;
        }
        //填充空格
        for i in s.len()..max_width as usize {
            s.push(' ');
        }
        ans.push(s);
        ans
    }
}

fn main() {
    let s = vec!["What", "must", "be", "acknowledgment", "shall", "be"];
    let mut ss = vec!();
    for i in s {
        ss.push(i.to_string());
    }
    let ans = Solution::full_justify(ss, 16);
    println!("{:?}", ans);
}
