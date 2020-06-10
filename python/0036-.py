class Solution(object):
    def isValidSudoku(self, board):
        """
        :type board: List[List[str]]
        :rtype: bool
        """
        for i in range(9):
            ma=set()
            for j in range(9):
                if board[i][j]=='.':continue
                if board[i][j] not in ma:
                    ma.add(board[i][j])
                else:
                    return False
            ma.clear()
            for j in range(9):
                if board[j][i]=='.':continue
                if board[j][i] not in ma:
                    ma.add(board[j][i])
                else:
                    return False
            ma.clear()
            r=i//3
            c=i%3
            x,y=r*3,c*3
            for j in range(3):
                for k in range(3):
                    if board[x+j][y+k] =='.':continue
                    if board[x+j][y+k] not in ma:
                        ma.add(board[x+j][y+k])
                    else:
                        return False
        return True