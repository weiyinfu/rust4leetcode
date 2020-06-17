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

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, mut word_list: Vec<String>) -> i32 {
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
            return 0;
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
        if dis[target] == -1 {
            return 0;
        }
        dis[target] + 1
    }
}