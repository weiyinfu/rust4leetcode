struct Solution;
// https://leetcode-cn.com/problems/regular-expression-matching/solution/yi-bu-dao-wei-zhi-jie-an-zheng-ze-biao-da-shi-de-s/

fn str2vec(s: &String) -> Vec<char> {
    return s.chars().collect::<Vec<char>>();
}

fn starts(a: &Vec<char>, p: &Vec<char>, i: usize) -> bool {
    let mut head = vec!();
    head.push((0, i));
    while !head.is_empty() {
        //当队列不为空
        let (h, j) = head.pop().unwrap();
        if h >= p.len() {
            return true;
        }
        if j >= a.len() {
            return false;
        }
        let ch = a[j];//当前字符
        // println!("head = {} j={}", h, j);
        // println!("{:?}", head);
        // println!("current char ={} ch={}", ch, p[h]);
        // println!("=======");
        if p[h] == '.' {
            head.push((h + 1, j + 1));
        } else if p[h] == '*' {
            //正在吃多个
            let c = a[j - 1];
            if c != ch {
                //*结束了
                head.push((h + 1, j));//执行下一个字符
            } else {
                //匹配上了，有两种决策，一种向后走，一种停在当地
                //停在原地
                head.push((h, j + 1));
                //向后走
                head.push((h + 1, j));
            }
        } else {
            if p[h] == ch {
                //匹配上了，往后走
                if h + 1 < p.len() && p[h + 1] == '*' {
                    head.push((h + 1, j + 1));
                    head.push((h + 2, j));
                } else {
                    head.push((h + 1, j + 1));
                }
            } else {
                //p[h]!=ch，没有匹配上，这时，可能是遇到*了
                if h + 1 < p.len() && p[h + 1] == '*' {
                    head.push((h + 2, j));//跳跃
                } else {
                    //彻底失配了
                }
            }
        }
    }
    false
}

fn index_of(a: &Vec<char>, p: &Vec<char>, beg: usize) -> usize {
    //寻找a中的p，从beg开始找，p中只包含.和*
    for i in beg..a.len() {
        if starts(a, p, i) {
            return i;
        }
    }
    a.len()
}

fn mat(a: Vec<char>, parts: Vec<Vec<char>>) -> bool {
    //a是否包含parts中的全部字符
    // println!("{:?} {:?}", parts, a);
    let mut i = 0usize;
    for p in &parts {
        println!("正在{:?}中寻找{:?}，从{}开始", a, p, i);
        let j = index_of(&a, p, i);
        if j == a.len() {
            println!("cannot find {:?}", p);
            return false;
        }
        i = j + p.len();
    }
    true
}

impl Solution {
    pub fn is_match(mut s: String, mut p: String) -> bool {
        s.push('$');
        p.push('$');
        s.insert(0, '$');
        p.insert(0, '$');
        let pp: Vec<&str> = p.split(".*").collect();
        let mut parts = vec!();
        for s in pp {
            //去掉子串的开头*和结尾*
            let mut now: Vec<char> = s.chars().collect();
            while !now.is_empty() && *now.last().unwrap() == '*' {
                //弹出结尾x*
                now.pop();
                now.pop();
            }
            while now.len() > 1 && now[1] == '*' {
                now.remove(0);
                now.remove(0);
            }
            if now.is_empty() {
                continue;
            }
            parts.push(now);
        }
        println!("{:?}", parts);
        mat(str2vec(&s), parts)
    }
}

fn main() {
    println!("{}", Solution::is_match("abc".to_string(), "ab*bc".to_string()));
}
