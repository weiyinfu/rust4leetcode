class Solution(object):
    def strStr(self, haystack, needle):
        """
        :type haystack: str
        :type needle: str
        :rtype: int
        """
        if not needle: return 0
        if not haystack: return -1

        if needle in haystack:
            return haystack.index(needle)
        return -1