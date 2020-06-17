use std::collections::HashMap;

struct Solution;

fn is_neibor(a: &String, b: &String) -> bool {
    let mut dif = 0;
    let mut is_neibor = true;
    for (c, cc) in a.chars().zip(b.chars()) {
        if c != cc {
            //如果不相等
            dif += 1;
            if dif > 1 {
                return false;
            }
        }
    }
    if dif == 0 {
        return false;
    }
    is_neibor
}

fn go(ans: &mut Vec<Vec<String>>,
      ind: usize,
      g: &Vec<Vec<usize>>,
      path: &mut Vec<usize>,
      words: &Vec<String>,
) {
    path.push(ind);
    if ind == 0 {
        ans.push(path.iter().map(|x| { words[*x].clone() }).collect());
        path.pop();
        return;
    }
    for p in g[ind].iter() {
        go(ans, *p, g, path, words);
    }
    path.pop();
}

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        word_list.insert(0, begin_word);
        let mut g = vec!(vec![]; word_list.len());
        for i in 0..word_list.len() {
            for j in i + 1..word_list.len() {
                let is = is_neibor(&word_list[i], &word_list[j]);
                if is {
                    g[i].push(j);
                    g[j].push(i);
                }
            }
        }
        let mut target = None;
        for i in 0..word_list.len() {
            if word_list[i] == end_word {
                target = Some(i);
            }
        }
        if target.is_none() {
            return vec!();
        }
        let target = target.unwrap();

        let mut q = vec!(0);
        let mut dis = vec!(-1; word_list.len());
        let mut gg = vec!(vec!(); word_list.len());
        dis[0] = 0;
        while !q.is_empty() {
            let now = q[0];
            q.remove(0);
            if now == target {
                break;
            }
            for des in g[now].iter() {
                if dis[*des] == -1 || dis[*des] == dis[now] + 1 {
                    gg[*des].push(now);
                    if dis[*des] == -1 {
                        //如果是第一次访问
                        q.push(*des);
                        dis[*des] = dis[now] + 1;
                    }
                }
            }
        }
        let mut ans = vec![];
        let mut path = vec!();
        go(&mut ans, target, &gg, &mut path, &word_list);
        for i in ans.iter_mut() {
            i.reverse();
        }
        ans
    }
}

fn main() {
    let beginWord = "hit".to_string();
    let endWord = "cog".to_string();
    let wordList = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string()];
    println!("{:?}", Solution::find_ladders(beginWord, endWord, wordList));
}
