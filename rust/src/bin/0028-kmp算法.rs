struct Solution;


fn build(s: &Vec<char>) -> Vec<i32> {
    let mut a = Vec::<i32>::with_capacity(s.len());
    for i in 0..s.len() + 1 {
        a.push(0);
    }
    let mut i = 0i32;
    let mut j = 1i32;
    a[0] = -1;
    a[1] = 0;
    while (j as usize) < s.len() {
        if i < 0 || s[i as usize] == s[j as usize] {
            //可以前进了
            i += 1;
            j += 1;
            a[j as usize] = i as i32;//在j处失配之后，可以去往i处
        } else {
            i = a[i as usize];
        }
    }
    a
}

fn search(a: Vec<i32>, s: Vec<char>, p: Vec<char>) -> i32 {
    let mut i = 0i32;
    let mut j = 0i32;
    while (j as usize) < s.len() {
        if s[j as usize] == p[i as usize] {
            i += 1;
            j += 1;
            if i as usize == p.len() {
                return (j as usize - p.len()) as i32;
            }
        } else {
            if i <= 0 {
                j += 1;
                i = 0;
            } else {
                i = a[i as usize];
            }
        }
    }
    -1
}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let a: Vec<char> = haystack.chars().collect();
        let b: Vec<char> = needle.chars().collect();
        let ta = build(&b);
        println!("{:?}", ta);
        return search(ta, a, b);
    }
}

fn main() {
    println!("{}", Solution::str_str("hllllo".to_string(), "llo".to_string()));
}