------------------
title = "leetcode48:矩阵旋转"
publishTime = "2017-09-24 12:35:00"
id = "7588480"
tags = [ "算法", "leetcode",]

--------------

[题目链接](https://leetcode.com/problems/rotate-image/description/)

输入一个N×N的方阵，要求不开辟新空间，实现矩阵旋转。

将点(x,y)绕原点顺时针旋转90度，变为（y,-x）。原来的（-y,x）变为（x,y）

```python
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
```
        