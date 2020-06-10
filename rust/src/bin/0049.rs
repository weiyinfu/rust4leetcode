struct Solution;

use std::collections::HashMap;
use std::iter::FromIterator;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ma = HashMap::new();
        for i in &strs {
            let mut chars = i.chars().collect::<Vec<char>>();
            chars.sort();
            let k = String::from_iter(chars);
            if !ma.contains_key(&k) {
                ma.insert(k, vec!(i.clone()));
            } else {
                let ans = ma.get_mut(&k).unwrap();
                ans.push(i.clone());
            }
        }
        let mut ans = vec!();
        for i in ma.values() {
            ans.push(i.to_owned());
        }
        ans
    }
}

fn main() {
    println!("haha");
}