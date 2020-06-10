struct Solution;

fn str2vec(s: &String) -> Vec<char> {
    return s.chars().collect::<Vec<char>>();
}

fn splitStar(p: &String) -> Vec<Vec<char>> {
    let chars: Vec<char> = str2vec(p);
    let mut a = Vec::<Vec<char>>::new();
    let mut now = Vec::<char>::new();
    for i in 0..chars.len() {
        if chars[i] == '*' {
            if now.len() > 0 {
                a.push(now.to_owned());
                now = Vec::<char>::new();
            }
            continue;
        }
        now.push(chars[i]);
    }
    if !now.is_empty() {
        a.push(now.to_owned());
    }
    return a;
}

fn startsWith(a: &Vec<char>, p: &Vec<char>, ind: i32) -> bool {
    //判断a[ind:]是否以p开头
    let mut i = 0;
    while i < p.len() {
        let ii = (ind + i as i32) as usize;
        if ii > a.len() {
            return false;
        }
        if a[ii] != p[i] || p[i] == '.' {
            return false;
        }
        i += 1;
    }
    true
}

fn indexOf(a: &Vec<char>, p: &Vec<char>, ind: i32) -> i32 {
    //p在a中的下标，从ind处开始搜索
    if ind > a.len() as i32 {
        return -1;
    }
    let mut i = ind;
    while i < a.len() as i32 {
        if startsWith(a, p, i) {
            return i;
        }
        i += 1;
    }
    -1
}

fn mat(a: Vec<char>, parts: Vec<Vec<char>>) -> bool {
    //a是否包含parts中的全部字符
    // println!("{:?} {:?}", parts, a);
    let mut i = 0;
    for p in &parts {
        let j = indexOf(&a, &p, i);
        if j == -1 {
            return false;
        }
        i = j + p.len() as i32;
    }
    true
}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut pp = p.clone();
        pp.push('$');//最后避免为*，所以追加一个魔法字符
        let mut ss = s.clone();
        ss.push('$');
        let parts = splitStar(&pp);
        mat(str2vec(&ss), parts)
    }
}

fn main() {
    let s = "aa".to_string();
    let p = "a*".to_string();
    println!("{}", Solution::is_match(s, p));
    println!("{}", Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
}
