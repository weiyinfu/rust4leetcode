struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let mut last_is = false;
        let mut a = String::new();
        for i in s.trim().chars() {
            let is = i >= '0' && i <= '9';
            if is {
                if last_is {
                    continue;
                }
                last_is = true;
                a.push('$');
            } else {
                last_is = false;
                a.push(i);
            }
        }
        let mut integers = vec!("$".to_string());
        let mut floats = vec!("$.$".to_string(),
                              ".$".to_string(),
                              "$.".to_string(), );
        for part in [&mut integers, &mut floats].iter_mut() {
            let fixed = part.clone();
            for op in "+-".chars() {
                for j in &fixed {
                    part.push(op.to_string() + j.clone().as_str());
                }
            }
        }
        let mut nums: Vec<String> = vec!();
        for i in [&integers, &floats].iter() {
            for j in *i {
                nums.push(j.clone());
            }
        }
        let fixed = nums.clone();
        for i in fixed {
            for e in ['e', 'E'].iter() {
                for j in &integers {
                    let mut z = i.clone();
                    z.push(*e);
                    z.push_str(j.clone().as_str());
                    nums.push(z);
                }
            }
        }
        for i in nums {
            if a == i {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("{}", Solution::is_number(".6e+3 ".to_string()));
}