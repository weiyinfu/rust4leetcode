class Solution(object):
    def generate(self, numRows):
        """
        :type numRows: int
        :rtype: List[List[int]]
        """
        ans=[[0]*numRows for i in range(numRows)]
        for i in range(numRows):
            ans[i][0]=1
        for i in range(1,numRows):
            for j in range(1,i+1):
                ans[i][j]=ans[i-1][j]+ans[i-1][j-1]
        a=[ans[i][:(i+1)]for i in range(numRows)]
        return a