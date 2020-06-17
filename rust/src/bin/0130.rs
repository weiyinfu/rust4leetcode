struct Solution;

fn go(visited: &mut Vec<Vec<bool>>, x: usize, y: usize, a: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut q = vec!((x as i32, y as i32));
    let direction = [(0i32, 1i32), (0, -1), (-1, 0), (1, 0)];
    let mut ans = vec!();
    let mut died = true;
    while !q.is_empty() {
        let (i, j) = q.pop().unwrap();
        ans.push((i as usize, j as usize));
        for di in direction.iter() {
            let (xx, yy) = (i + (*di).0, j + (*di).1);
            if xx >= 0 && yy >= 0 && xx < visited.len() as i32 && yy < visited[0].len() as i32 {
                //如果合法
                if !visited[xx as usize][yy as usize]
                    && a[xx as usize][yy as usize] == 'O' {
                    visited[xx as usize][yy as usize] = true;
                    q.push((xx, yy));
                }
            } else {
                died = false;
            }
        }
    }
    if !died {
        return vec!();
    }
    ans
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() == 0 {
            return;
        }
        if board[0].len() == 0 {
            return;
        }
        let rows = board.len();
        let cols = board[0].len();
        let mut visited = vec!(vec!(false; cols); rows);
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' && !visited[i][j] {
                    let res = go(&mut visited, i, j, &board);
                    for (x, y) in res {
                        board[x][y] = 'X';
                    }
                }
            }
        }
    }
}

fn main() {
    let ma = "
    XXXX
    XOOX
    XXOX
    XOXX";
    let a: Vec<String> = ma.split_whitespace().map(|x| x.to_string()).collect();
    let mut a: Vec<Vec<char>> = a.iter().map(|x| {
        x.chars().collect::<Vec<char>>()
    }).collect();
    Solution::solve(&mut a);
    println!("{:?}", a);
}