struct Solution;

fn go(a: &Vec<String>, b: &mut Vec<String>, now: &mut String, ind: usize) {
    if ind == a.len() {
        b.push(now.clone());
        return;
    }
    for i in a[ind].chars() {
        now.push(i);
        go(&a, b, now, ind + 1);
        now.pop();
    }
}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let a: Vec<String> = "abc def ghi jkl mno pqrs tuv wxyz".split_whitespace().map(|x| { x.to_string() }).collect();
        let mut v = Vec::<String>::new();
        for i in digits.chars() {
            let ind = i as i32 - '2' as i32;
            v.push(a[ind as usize].clone());
        }
        let mut ans = Vec::<String>::new();
        go(&v, &mut ans, &mut "".to_string(), 0);
        ans
    }
}

fn main() {
    let res = Solution::letter_combinations("23".to_string());
    println!("{:?}", res);
}