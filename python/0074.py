class Solution:
    def solve(self, matrix, x, y, xx, yy, target):
        if target > matrix[xx - 1][yy - 1] or target < matrix[x][y]: return False
        if x + 1 >= xx and y + 1 >= yy:
            if matrix[x][y] == target:
                return True
            else:
                return False
        mx = (x + xx) >> 1
        my = (y + yy) >> 1
        if matrix[mx][my] > target:
            ans = self.solve(matrix, x, y, mx, my, target)
        else:
            ans = self.solve(matrix, mx, my, xx, yy, target)
        if ans: return True
        ans = self.solve(matrix, mx, y, xx, my, target) or self.solve(matrix, x, my, mx, yy, target)
        return ans

    def searchMatrix(self, matrix, target):
        """
        :type matrix: List[List[int]]
        :type target: int
        :rtype: bool
        """
        if not matrix: return False
        r = len(matrix)
        if not r: return False
        c = len(matrix[0])
        if not c: return False
        return self.solve(matrix, 0, 0, r, c, target)


if __name__ == '__main__':
    print(Solution().searchMatrix([[1, 3]], 1))
