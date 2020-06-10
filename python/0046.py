class Solution(object):
    def __init__(self):
        self.ans=[]
    def permute(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """
        def go(nums,beg):
            if beg==len(nums):
                self.ans.append(nums[:])
                return
            for i in range(beg,len(nums)):
                nums[beg],nums[i]=nums[i],nums[beg]
                go(nums,beg+1)
                nums[beg],nums[i]=nums[i],nums[beg]
        go(nums,0)
        return self.ans