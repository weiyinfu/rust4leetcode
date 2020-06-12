struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }
        if matrix[0].len() == 0 {
            return false;
        }
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        loop {
            if matrix[i][j] < target {
                i += 1;
                if i == matrix.len() {
                    return false;
                }
            } else if matrix[i][j] > target {
                if j == 0 {
                    return false;
                }
                j -= 1;
            } else {
                return true;
            }
        }
    }
}

fn main() {
    let matrix = vec![
        vec![1, 3, 5, 7],
        vec![10, 11, 16, 20],
        vec![23, 30, 34, 50]
    ];
    let ans = Solution::search_matrix(matrix, 17);
    println!("{}", ans);
}