struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let ans: Vec<&str> = path.split("/").collect();
        let mut sta = vec![];
        for i in ans {
            if i.len() == 0 {
                continue;
            }
            if i == ".".to_string() {
                continue;
            }
            if i == "..".to_string() {
                sta.pop();
                continue;
            }
            sta.push(i);
        }
        let mut a: String = sta.join("/");
        a.insert(0, '/');
        a
    }
}

fn main() {
    let ans = Solution::simplify_path("/haha/../one/./three".to_string());
    println!("{}", ans);
}