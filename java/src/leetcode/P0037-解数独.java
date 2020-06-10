import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

class Solution {

class Point {
    int x, y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }
}

int getCard(int x, int y) {
    return (x / 3) * 3 + (y / 3);
}

void put(int[] state, int x, int y, int num, char[][] board) {
    state[x] |= (1 << num);
    state[y + 9] |= (1 << num);
    int card = getCard(x, y);
    state[card + 18] |= (1 << num);
    board[x][y] = (char) ('0' + num);
}

void rm(int[] state, int x, int y, int num, char[][] board) {
    state[x] &= ~(1 << num);
    state[y + 9] &= ~(1 << num);
    int card = getCard(x, y);
    state[card + 18] &= ~(1 << num);
    board[x][y] = '.';
}

boolean ok(int[] state, int x, int y, int num) {
    int mask = 1 << num;
    if ((state[x] & mask) != 0) return false;
    else if ((state[y + 9] & mask) != 0) return false;
    else if ((state[getCard(x, y) + 18] & mask) != 0) return false;
    else return true;
}

void pause() {
    try {
        System.in.read();
    } catch (IOException e) {
        e.printStackTrace();
    }
}

boolean go(int[] state, List<Point> spaces, int ind, char[][] board) {
    if (ind == spaces.size()) {
//        printBoard(board);
        return true;
    }

    Point pos = spaces.get(ind);
//    System.out.println(" pos: " + pos.x + " " + pos.y);
//    printBoard(board);
//    pause();
    for (int v = 1; v <= 9; v++) {
        if (ok(state, pos.x, pos.y, v)) {
            put(state, pos.x, pos.y, v, board);
            boolean over = go(state, spaces, ind + 1, board);
            if (over) return true;
            rm(state, pos.x, pos.y, v, board);
        }
    }
    return false;
}

public void solveSudoku(char[][] board) {
    ArrayList<Point> spaces = new ArrayList<>();
    int[] state = new int[27];
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (board[i][j] == '.') {
                spaces.add(new Point(i, j));
            } else {
                int num = board[i][j] - '0';
                put(state, i, j, num, board);
            }
        }
    }
    go(state, spaces, 0, board);

}

static void printBoard(char[][] board) {
    for (char[] a : board) {
        for (char c : a) {
            System.out.print(c);
        }
        System.out.println();
    }
}

public static void main(String[] args) {
    char[][] board = new char[][]{{'5', '3', '.', '.', '7', '.', '.', '.', '.'}, {'6', '.', '.', '1', '9', '5', '.', '.', '.'}, {'.', '9', '8', '.', '.', '.', '.', '6', '.'}, {'8', '.', '.', '.', '6', '.', '.', '.', '3'}, {'4', '.', '.', '8', '.', '3', '.', '.', '1'}, {'7', '.', '.', '.', '2', '.', '.', '.', '6'}, {'.', '6', '.', '.', '.', '.', '2', '8', '.'}, {'.', '.', '.', '4', '1', '9', '.', '.', '5'}, {'.', '.', '.', '.', '8', '.', '.', '7', '9'}};
    new Solution().solveSudoku(board);
}
}