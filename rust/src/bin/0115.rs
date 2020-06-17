struct Solution;

fn go(a: &mut Vec<Vec<i32>>, i: usize, j: usize, s: &Vec<char>, t: &Vec<char>) -> i32 {
    //go表示从i到j
    if a[i][j] != -1 {
        return a[i][j];
    }
    if i == s.len() {
        if j == t.len() {
            a[i][j] = 1;
            return 1;
        } else {
            a[i][j] = 0;
            return 0;
        }
    }
    if j == t.len() {
        a[i][j] = 1;
        return 1;
    }
    let mut now = 0;
    if s[i] == t[j] {
        //如果相等
        now += go(a, i + 1, j + 1, s, t);
    }
    now += go(a, i + 1, j, s, t);
    a[i][j] = now;
    now
}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let mut a = vec!(vec!(-1; t.len() + 1); 1 + s.len());
        let ss: Vec<char> = s.chars().collect();
        let tt = t.chars().collect();
        go(&mut a, 0, 0, &ss, &tt)
    }
}

fn main() {
    let ans = Solution::num_distinct("babgbag".to_string(), "bag".to_string());
    println!("{}", ans);
}