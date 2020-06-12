struct Solution;

fn go(x: usize, y: usize,
      board: &mut Vec<Vec<char>>,
      w: &Vec<char>, ind: usize) -> bool {
    if ind == w.len() {
        return true;
    }
    let ch = board[x][y];
    if ch != w[ind] {
        return false;
    }
    if ind == w.len() - 1 {
        return true;
    }
    static DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let rows = board.len() as i32;
    let cols = board[0].len() as i32;
    let legal = |x, y| {
        x >= 0 && y >= 0 && x < rows as i32 && y < cols
    };

    board[x][y] = '$';
    for i in DIRECTIONS.iter() {
        let (xx, yy) = (x as i32 + i.0, y as i32 + i.1);
        if !legal(xx, yy) {
            continue;
        }
        let res = go(xx as usize, yy as usize, board, w, ind + 1);
        if res {
            return true;
        }
    }
    board[x][y] = ch;
    false
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        if board.len() == 0 {
            return false;
        }
        if board[0].len() == 0 {
            return false;
        }
        let rows = board.len();
        let cols = board[0].len();
        let w: Vec<char> = word.chars().collect();
        for i in 0..rows {
            for j in 0..cols {
                if go(i, j, &mut board, &w, 0) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let board =
        vec![
            vec!['A'],
        ];
    let word = "A".to_string();
    let ans = Solution::exist(board, word);
    println!("{}", ans);
}