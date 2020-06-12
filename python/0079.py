class Solution:
    def exist(self, board, word):
        """
        :type board: List[List[str]]
        :type word: str
        :rtype: bool
        """
        w=len(board)
        h=len(board[0])
        def legal(x,y):
            return x>=0 and y>=0 and x<w and y<h
        def ok(x,y,ind,vis):
            if ind==len(word)-1:
                return True
            vis[x][y]=True
            for dx,dy in ((-1,0),(1,0),(0,-1),(0,1)):
                m,n=x+dx,y+dy
                if legal(m,n) and not vis[m][n] and word[ind+1]==board[m][n]:
                    ans=ok(m,n,ind+1,vis)
                    if ans:
                        return True
            vis[x][y]=False
            return False
        for i in range(w):
            for j in range(h):
                if word[0]==board[i][j]:
                    if ok(i,j,0,[[False]*h for i in range(w)]):
                        return True
        return False