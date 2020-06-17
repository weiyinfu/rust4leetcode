class Solution(object):
    def getRow(self, rowIndex):
        """
        :type rowIndex: int
        :rtype: List[int]
        """
        a=[1]
        s=1
        for i in range(1,rowIndex+1):
            s=s*(1+rowIndex-i)/i
            a.append(s)
        print(a)
        return a