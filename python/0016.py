class Solution(object):
    def __init__(self):
        self.ans=0xffffffffff
    def threeSumClosest(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: int
        """
        nums=sorted(nums)
        def update(i,j,k):
            now_ans=nums[i]+nums[j]+nums[k]
            if abs(self.ans-target)>abs(now_ans-target):
                self.ans=now_ans
        for i in range(len(nums)-2):
            k=len(nums)-1
            for j in range(i+1,len(nums)-1):
                while k>j and nums[i]+nums[j]+nums[k]>target:
                    k-=1
                if k<j:
                    update(i,j,j+1)
                    break
                if k>j and k<len(nums):
                    update(i,j,k)
                if k>=j and k+1<len(nums):
                    update(i,j,k+1)
        return self.ans