class Solution(object):
    def intToRoman(self, num):
        """
        :type num: int
        :rtype: str
        """
        ma={
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
            'M':1000,
        }
        ans=''
        if num>=1000:

            ans+='M'*(num//1000)
            num%=1000
        if num>=900:
            ans+='CM'
            num%=100
        if num>=500:
            ans+='D'+'C'*(num//100-5)
            num%=100
        if num>=400:
            ans+='CD'
            num%=100
        if num>=100:
            ans+='C'*(num//100)
            num%=100
        if num>=90:
            ans+="XC"
            num%=10
        if num>=50:
            ans+='L'+'X'*(num//10-5)
            num%=10
        if num>=40:
            ans+="XL"
            num%=10
        if num>=10:
            ans+='X'*(num//10)
            num%=10
        if num>=9:
            ans+='IX'
            num=0
        if num>=5:
            ans+='V'+'I'*(num-5)
            num=0
        if num>=4:
            ans+='IV'
            num=0
        if num>0:
            ans+='I'*num
            num=0
        return ans