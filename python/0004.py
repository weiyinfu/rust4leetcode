import math


class Solution(object):
    def findpos(self, x, nums1, nums2):
        a, b = 0, 0
        l = len(nums1) + len(nums2)
        while a + b < x:
            # print(a, b, '==')
            if a == len(nums1):
                b = x - a
                break
            if b == len(nums2):
                a = x - b
                break
            step = math.ceil((x - a - b) / 2)
            pos1 = min(a + step, len(nums1))
            pos2 = min(b + step, len(nums2))
            v1 = nums1[pos1 - 1]
            v2 = nums2[pos2 - 1]
            if v1 < v2:
                a = pos1
            else:
                b = pos2
        if a == 0:
            return nums2[b - 1]
        elif b == 0:
            return nums1[a - 1]
        else:
            return max(nums1[a - 1], nums2[b - 1])

    def findMedianSortedArrays(self, nums1, nums2):
        """
        :type nums1: List[int]
        :type nums2: List[int]
        :rtype: float
        """
        l = len(nums1) + len(nums2)
        return (self.findpos(math.floor((l + 1) / 2), nums1, nums2) + self.findpos(math.ceil((l + 1) / 2), nums1,nums2)) / 2

