struct Solution;

fn tos(a: &Vec<Vec<bool>>) -> Vec<String> {
    let mut s = Vec::new();
    for x in 0..a.len() {
        let mut now = String::new();
        for y in 0..a[x].len() {
            now.push(if a[x][y] { 'Q' } else { '.' })
        }
        s.push(now);
    }
    s
}

fn go(ans: &mut i32, board: &mut Vec<Vec<bool>>,
      x: usize, col: u64, zig: u64, zag: u64,
      put: usize) {
    if x == board.len() {
        //到头了
        *ans += 1;
        return;
    }
    for y in 0..board.len() {
        //在x，y处是否放置
        let zig_id = if x > y { board.len() + x - y } else { board.len() - (y - x) };
        let yy = board.len() - y;
        let zag_id = if x > yy { board.len() + (x - yy) } else { board.len() - (yy - x) };
        let col_has = col & (1 << y) as u64;
        let zig_has = zig & (1 << zig_id) as u64;
        let zag_has = zag & (1 << zag_id) as u64;
        if (col_has | zig_has | zag_has) == 0 {
            //可以放置
            board[x][y] = true;
            go(ans, board, x + 1,
               col | (1 << y) as u64, zig | (1 << zig_id) as u64, zag | (1 << zag_id) as u64,
               put + 1);
            board[x][y] = false;
        }
    }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut board: Vec<Vec<bool>> = vec![];
        for i in 0..n {
            let mut row = vec!();
            for j in 0..n {
                row.push(false);
            }
            board.push(row);
        }
        let mut ans = 0;
        go(&mut ans, &mut board, 0, 0, 0, 0, 0);
        ans
    }
}

fn main() {
    let ans = Solution::total_n_queens(8);
    println!("{}",ans);
}