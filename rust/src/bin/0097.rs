struct Solution;

fn ok(a: &Vec<char>, b: &Vec<char>, c: &Vec<char>, i: usize, j: usize, f: &mut Vec<Vec<i32>>) -> bool {
    if f[i][j] != -1 {
        return f[i][j] != 0;
    }
    let c_ind = i + j;//剩下的字符串的长度
    if c_ind == c.len() {
        if i == a.len() && j == b.len() {
            f[i][j] = 1;
            return true;
        } else {
            f[i][j] = 0;
            return false;
        }
    }
    let k = c[c_ind];
    if i < a.len() && a[i] == k {
        if ok(a, b, c, i + 1, j, f) {
            f[i][j] = 1;
            return true;
        }
    }
    if j < b.len() && b[j] == k {
        if ok(a, b, c, i, j + 1, f) {
            f[i][j] = 1;
            return true;
        }
    }
    f[i][j] = 0;
    false
}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let mut i = 0;
        let mut j = 0;
        let a: Vec<char> = s1.chars().collect();
        let b: Vec<char> = s2.chars().collect();
        let c: Vec<char> = s3.chars().collect();
        if c.len() != a.len() + b.len() {
            return false;
        }
        let mut f = vec!(vec!(-1; b.len() + 1); a.len() + 1);
        ok(&a, &b, &c, 0, 0, &mut f)
    }
}

fn main() {
    let ans = Solution::is_interleave("a".to_string(),
                                      "".to_string(),
                                      "a".to_string());
    println!("{}", ans);
}