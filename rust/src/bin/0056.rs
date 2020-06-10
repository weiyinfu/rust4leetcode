struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut points = vec!();
        for i in intervals {
            points.push((i[0], 1));
            points.push((i[1], -1));
        }
        points.sort_by_key(|x| { (x.0, -x.1) });
        let mut ans: Vec<Vec<i32>> = vec!();
        let mut thick = -1;
        let passwd = 0x23423423;
        let mut beg = passwd;
        println!("{:?}", points);
        for i in points {
            if beg == passwd {
                beg = i.0;
                thick = 1;
                continue;
            }
            thick += i.1;
            if thick == 0 {
                ans.push(vec!(beg, i.0));
                beg = passwd;
            }
        }
        ans
    }
}

fn main() {
    let a = vec![
        vec![1, 3],
        vec![2, 6],
        vec![8, 10],
        vec![15, 18]];
    let x = Solution::merge(a);
    println!("{:?}", x);
}