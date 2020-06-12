class Solution:
    def removeDuplicates(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        last=None
        cnt=0
        ans=0
        for i in nums:
            if last==i:
                cnt+=1
                if cnt<2:
                    nums[ans]=i
                    ans+=1
            else:
                nums[ans]=i
                ans+=1
                last=i
                cnt=0
        return ans