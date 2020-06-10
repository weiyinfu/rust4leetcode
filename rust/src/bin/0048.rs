struct Solution;


impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        if matrix.len() == 0 {
            return;
        }
        let sz = matrix.len();
        let get = |x: usize, y: usize| {
            (y, sz - 1 - x)
        };
        for x in 0..=sz / 2 {
            for y in x..sz - 1 - x {
                let mut temp = matrix[x][y];
                let (mut xx, mut yy) = (x, y);
                for i in 0..3 {
                    let (mut xxx, mut yyy) = get(xx, yy);
                    let nex = matrix[xxx][yyy];
                    matrix[xxx][yyy] = temp;
                    temp = nex;
                    xx = xxx;
                    yy = yyy;
                }
                matrix[x][y] = temp;
            }
        }
    }
}

fn main() {
    let mut a = vec![
        vec![1, 2, ],
        vec![4, 5, ],
    ];
    Solution::rotate(&mut a);
    println!("{:?}", a);
}