class Solution(object):
    def rotate(self, matrix):
        """
        :type matrix: List[List[int]]
        :rtype: void Do not return anything, modify matrix in-place instead.
        """
        n = len(matrix)
        for i in range(n - 1):
            for j in range(i, n - 1 - i):
                t = matrix[i][j]
                x, y = i, j
                for k in range(3):
                    tx = n - y - 1
                    ty = x
                    matrix[x][y] = matrix[tx][ty]
                    x, y = tx, ty
                matrix[x][y] = t