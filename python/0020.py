class Solution(object):
    def isValid(self, s):
        """
        :type s: str
        :rtype: bool
        """
        sta=[]
        ma={')':'(','}':'{',']':'['}
        for i in s:
            if i in '([{':
                sta.append(i)
            elif i in ')]}':
                if len(sta)==0:
                    return False
                if sta[-1]==ma[i]:
                    del sta[-1]
                else:
                    return False
        return len(sta)==0