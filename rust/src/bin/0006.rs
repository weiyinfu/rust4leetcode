struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let chars: Vec<char> = s.chars().collect();
        if num_rows == 1 {
            return s;
        }
        let mut ss = String::new();
        let period = num_rows * 2 - 2;
        for i in 0..num_rows {
            if i == 0 {
                //第一行
                let mut j = 0;
                while j < chars.len() as i32 {
                    ss.push(chars[j as usize]);
                    j += period;
                }
            } else if i == num_rows - 1 {
                //最后一行
                let mut j = num_rows - 1;
                while j < chars.len() as i32 {
                    ss.push(chars[j as usize]);
                    j += period;
                }
            } else {
                //中间部分，每行都有两个字符
                let mut delta = vec!((num_rows - 1 - i) * 2, i * 2);
                let mut j = i;
                let mut di = 0;
                while j < chars.len() as i32 {
                    //j不停往后走
                    ss.push(chars[j as usize]);
                    j += delta[di];
                    di = 1 - di;
                }
            }
        }
        return ss;
    }
}

fn main() {
    let ans = Solution::convert("ABC".to_string(), 1);
    println!("{}", ans);
}