class Solution(object):
    def merge(self, nums1, m, nums2, n):
        """
        :type nums1: List[int]
        :type m: int
        :type nums2: List[int]
        :type n: int
        :rtype: void Do not return anything, modify nums1 in-place instead.
        """
        if not nums2:return
        a,b=0,0
        while a<m and b<n:
            if nums1[a]<=nums2[b]:
                a+=1
            else:
                nums1[a],nums2[b]=nums2[b],nums1[a]
                j=b
                while j+1<len(nums2) and nums2[j]>nums2[j+1]:
                    nums2[j+1],nums2[j]=nums2[j],nums2[j+1]
                    j+=1
        while 1:
            nums1[a]=nums2[b]
            a+=1
            b+=1
            if b==n:
                break