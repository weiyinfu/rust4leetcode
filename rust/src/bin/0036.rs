struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut had = [false; 9];
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let ch = (board[i][j] as usize) - ('1' as usize);
                if had[ch] {
                    return false;
                }
                had[ch] = true;
            }
            let mut col = [false; 9];
            for j in 0..9 {
                if board[j][i] == '.' {
                    continue;
                }
                let ch = (board[j][i] as usize) - ('1' as usize);
                if col[ch] {
                    return false;
                }
                col[ch] = true;
            }
            let mut region = [false; 9];
            let x = i / 3 * 3;
            let y = i % 3 * 3;
            for xx in x..x + 3 {
                for yy in y..y + 3 {
                    if board[xx][yy] == '.' {
                        continue;
                    }
                    let ch = (board[xx][yy] as usize) - ('1' as usize);
                    if region[ch] {
                        return false;
                    }
                    region[ch] = true;
                }
            }
        }
        true
    }
}

fn main() {
    let ans = Solution::is_valid_sudoku(vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']]);
    println!("{}", ans);
}