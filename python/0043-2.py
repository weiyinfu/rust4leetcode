class Solution(object):
    def multiply(self, num1, num2):
        """
        :type num1: str
        :type num2: str
        :rtype: str
        """
        num1=num1[::-1]
        num2=num2[::-1]
        ans=[0]*(len(num1)+len(num2))
        for i in range(len(num1)):
            for j in range(len(num2)):
                ans[i+j]+=int(num1[i])*int(num2[j])
        enter=0
        for i in range(len(ans)):
            ans[i]+=enter
            enter=int(ans[i]/10)
            ans[i]%=10
        s=''
        i=len(ans)-1
        while i>=0 and ans[i]==0:
            i-=1
        while i>=0:
            s+=str(ans[i])
            i-=1
        if not s:
            return '0'
        return s