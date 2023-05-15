------------------
title = "leetcode679:24Game"
publishTime = "2017-09-22 04:05:00"
id = "7574197"
tags = [ "算法", "leetcode",]

--------------

[题目链接](https://leetcode.com/problems/24-game/description/)

考虑1，5，5，5这种情况，有：`5*(5-1/5)=24`所以除法必须自定义运算才行。

```python3
class Num:
    def __init__(self,up,down=1):
        self.up=up
        self.down=down
    def gcd(self,x,y):
        return x if y==0 else self.gcd(y,x%y)
    def simple(self):
        if self.up==0:
            return 
        d=self.gcd(self.up,self.down)
        self.up//=d
        self.down//=d
    def mul(self,x):
        res=Num(self.up*x.up,self.down*x.down)
        res.simple()
        return res
    def div(self,x):
        res=Num(self.up*x.down,self.down*x.up)
        res.simple()
        return res
    def add(self,x):
        res=Num(self.up*x.down+self.down*x.up,self.down*x.down)
        res.simple()
        return res
    def sub(self,x):
        res=Num(self.up*x.down-self.down*x.up,self.down*x.down)
        res.simple()
        return res
    def __str__(self):
        if self.down==1:return str(self.up)
        return "{}/{}".format(self.up,self.down)
class Solution(object):
    def judgePoint24(self, nums):
        """
        :type nums: List[int]
        :rtype: bool
        """
        def op(x,y,o):
            if o==0:
                return x.mul(y)
            elif o==1:
                return x.div(y)
            elif o==2:
                return x.add(y)
            elif o==3:
                return x.sub(y) 
            elif o==4:
                return y.sub(x)
            else:
                return y.div(x)
        def newar(a,i,j,z):
            ans=[z]
            for k in range(len(a)):
                if k!=i and k!=j:
                    ans.append(a[k])
            return ans
        def tos(x):
            return ','.join([str(i) for i in x])
        def go(nums):  
            if len(nums)==1:
                if nums[0].up==24 and nums[0].down==1:
                    return True
                else:
                    return False
            

            for i in range(len(nums)):
                for j in range(i+1,len(nums)):
                    for k in range(6):
                        z=op(nums[i],nums[j],k)
                        res=go(newar(nums,i,j,z))
                        if res:
                            return True
            return False
        nums=[Num(i)for i in nums]
        return go(nums)

```
        