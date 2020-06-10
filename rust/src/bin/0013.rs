struct Solution;

fn parse(a: &Vec<char>, i: i32, one: char, five: char, ten: char, base: i32) -> (i32, i32) {
    if i + 1 < a.len() as i32 && a[i as usize] == one && a[(i + 1) as usize] == ten {
        return (i + 2, 9 * base);
    }
    if i + 1 < a.len() as i32 && a[i as usize] == one && a[(i + 1) as usize] == five {
        return (i + 2, 4 * base);
    }
    let mut j = i;
    let mut s = 0;
    while j < a.len() as i32 {
        if a[j as usize] == five {
            s += 5 * base;
        } else if a[j as usize] == one {
            s += base;
        } else {
            break;
        }
        j += 1;
    }
    return (j, s);
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let m = [
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000)];
        let a: Vec<char> = s.chars().collect();
        let mut s = 0;
        let mut i = 0;
        let (i, v) = parse(&a, i, 'M', '$', '$', 1000);
        s += v;
        let (i, v) = parse(&a, i, 'C', 'D', 'M', 100);
        s += v;
        let (i, v) = parse(&a, i, 'X', 'L', 'C', 10);
        s += v;
        let (i, v) = parse(&a, i, 'I', 'V', 'X', 1);
        s += v;
        s
    }
}

fn main() {
    let res = Solution::roman_to_int("MCMXCIV".to_string());
    println!("{}", res);
}