class Solution(object):
    def countAndSay(self, n):
        """
        :type n: int
        :rtype: str
        """
        def cnt(s,ind):
            cnt=0
            i=ind
            while i<len(s) and s[ind]==s[i]:
                cnt+=1
                i+=1
            return cnt
        s="1"
        for i in range(n-1):
            j=0
            now=""
            while j<len(s):
                sz=cnt(s,j)
                now+=str(sz)+str(s[j])
                j+=sz
            s=now
        return s