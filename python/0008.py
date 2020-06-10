
class Solution(object):
    def myAtoi(self, str):
        """
        :type str: str
        :rtype: int
        """
        str=str.strip()
        if not str:
            return 0
        sig=1
        i=0
        while i< len(str):
            if i<len(str) and str[i]=='+':
                i+=1
                break
            elif str[i]=='-':
                sig*=-1
                i+=1
                break
            else:break
        value='0'
        while i<len(str):
            if str[i]in '0123456789':
                value+=str[i]
                i+=1
            else:
                break
        ans=int(value)*sig
        if ans>=2147483648:
            ans=2147483647
        if ans<-2147483648:
            ans=-2147483648
        return ans