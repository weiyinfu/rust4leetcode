use std::cmp::max;

struct Solution;

impl Solution {
    pub fn candy(mut ratings: Vec<i32>) -> i32 {
        let mut ind: Vec<usize> = (0..ratings.len()).collect();
        ind.sort_by_key(|x| ratings[*x]);
        let mut give = vec!(-1; ratings.len());
        for i in ind {
            give[i] = 1;
            if i > 0 && ratings[i] > ratings[i - 1] {
                give[i] = max(give[i], give[i - 1] + 1);
            }
            if i + 1 < give.len() && ratings[i] > ratings[i + 1] {
                give[i] = max(give[i], give[i + 1] + 1);
            }
        }
        give.iter().sum()
    }
}

fn main() {
    // let a = Solution::candy(vec!(1, 0, 2));
    // let a = Solution::candy(vec!(3, 2, 1, 2));
    // let a = Solution::candy(vec!(3, 2, 2, 3));
    // let a = Solution::candy(vec!(3, 2, 2, 1));
    let a = Solution::candy(vec!(1, 2, 2, 1));
    println!("{}", a);
}