class Solution(object):
    def nextPermutation(self, nums):
        """
        :type nums: List[int]
        :rtype: void Do not return anything, modify nums in-place instead.
        """
        def reverse(a,i,j):
            while i<j:
                a[i],a[j]=a[j],a[i]
                i+=1
                j-=1
        i=len(nums)-1
        while i>0 and nums[i]<=nums[i-1]:
            i-=1
        if i==0:
            reverse(nums,0,len(nums)-1)
            return
        reverse(nums,i,len(nums)-1)
        j=len(nums)-1
        while nums[j]>nums[i-1]:
            j-=1
        nums[i-1],nums[j+1]=nums[j+1],nums[i-1]