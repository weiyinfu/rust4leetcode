class Solution(object):
    def romanToInt(self, s):
        """
        :type s: str
        :rtype: int
        """
        a={
            'I':1,
            'IV':4,
            'V':5,
            'IX':9,
            'X':10,
            'XL':40,
            'L':50,
            'XC':90,
            'C':100,
            'CD':400,
            'D':500,
            'CM':900,
            'M':1000
        }
        ans=0
        i=0
        while i<len(s):
            v=None
            for j in a:
                if s[i:].startswith(j):
                    if v==None or len(j)>len(v):
                        v=j
            if v:
                i+=len(v)
                ans+=a[v]
        return ans