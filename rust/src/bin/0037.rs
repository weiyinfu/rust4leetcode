struct Solution;

fn rid(x: &usize, y: &usize) -> usize {
    return x / 3 * 3 + y / 3;
}

fn solve(a: &mut [[i32; 9]; 9]) -> bool {
    let mut row = [0; 9];
    let mut col = [0; 9];
    let mut region = [0; 9];
    let mut points = Vec::<(usize, usize, usize)>::new();
    //预处理
    for x in 0..9 {
        for y in 0..9 {
            let ch = a[x][y];
            let id = rid(&x, &y);
            if ch == 0 {
                points.push((x, y, id));
                continue;
            }
            row[x] |= (1 << ch);
            col[y] |= (1 << ch);
            region[id] |= (1 << ch);
        }
    }
    fn go(ind: usize, points: &Vec<(usize, usize, usize)>,
          a: &mut [[i32; 9]; 9],
          row: &mut [i32; 9],
          col: &mut [i32; 9],
          region: &mut [i32; 9]) -> bool {
        if ind == points.len() {
            return true;
        }
        let (x, y, reid) = points[ind];
        for i in 1..10 {
            let ch = 1 << i;
            if (row[x] & ch) > 0 || (col[y] & ch) > 0 || (region[reid] & ch) > 0 {
                continue;
            }
            row[x] |= ch;
            col[y] |= ch;
            region[reid] |= ch;
            a[x][y] = i;
            let res = go(ind + 1, points, a, row, col, region);
            if res {
                //找到答案之后立即返回
                return true;
            }
            let flag = -ch - 1;
            row[x] &= flag;
            col[y] &= flag;
            region[reid] &= flag;
        }
        false
    }
    return go(0, &points, a, &mut row, &mut col, &mut region);
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut a = [[0; 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    a[i][j] = 0;
                    continue;
                }
                a[i][j] = (board[i][j] as i32) - ('0' as i32);
            }
        }
        println!("{:?}", a);
        let res = solve(&mut a);
        if res {
            println!("有解");
        }
        for i in 0..9 {
            for j in 0..9 {
                board[i][j] = (('0' as u8 as i32) + a[i][j]) as u8 as char;
            }
        }
    }
}

fn main() {
    let mut p = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']];
    Solution::solve_sudoku(&mut p);
    println!("{:?}", p);
}