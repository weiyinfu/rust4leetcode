class Solution(object):
    def __init__(self):
        self.ans=[]
    def permuteUnique(self, nums):
        """
        :type nums: List[int]
        :rtype: List[List[int]]
        """

        def go(i):
            if i==len(nums):
                self.ans.append(nums[:])
                return
            ma=set()
            for j in range(i,len(nums)):
                if nums[j] not in ma:
                    ma.add(nums[j])
                    nums[i],nums[j]=nums[j],nums[i]
                    go(i+1)
                    nums[i],nums[j]=nums[j],nums[i]
        nums=sorted(nums)
        go(0)
        return self.ans