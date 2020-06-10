class Solution:
    def spiralOrder(self, matrix):
        """
        :type matrix: List[List[int]]
        :rtype: List[int]
        """
        a = []
        if not matrix:
            return a
        row = len(matrix)
        col = len(matrix[0])

        def legal(x, y):
            return row > x >= 0 and col > y >= 0

        di = ((0, 1), (1, 0), (0, -1), (-1, 0))
        d = 0
        vis = [[0] * col for _ in range(row)]
        x = 0
        y = 0
        while len(a) < row * col:
            vis[x][y] = True
            a.append(matrix[x][y])
            xx = di[d][0] + x
            yy = di[d][1] + y
            if not legal(xx, yy) or vis[xx][yy]:
                d = (d + 1) % 4
            x = x + di[d][0]
            y = y + di[d][1]
        return a
