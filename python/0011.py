class Solution(object):
    def maxArea(self, height):
        """
        :type height: List[int]
        :rtype: int
        """
        ans=0
        ma=0
        for i in range(len(height)):
            if height[i]<ma:continue
            else:ma=max(height[i],ma)
            j=len(height)-1
            while j>i and height[j]<height[i]:
                j-=1
            if height[j]>=height[i]:
                s=(j-i)*height[i]
                ans=max(s,ans)
        ma=0
        for i in range(len(height)-1,-1,-1):
            if height[i]<ma:continue
            else:ma=max(height[i],ma)
            j=0
            while j<i and height[j]<height[i]:
                j+=1
            if height[j]>=height[i]:
                s=(i-j)*height[i]
                ans=max(s,ans)
        return ans