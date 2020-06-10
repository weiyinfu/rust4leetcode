class Solution(object):
    def groupAnagrams(self, strs):
        """
        :type strs: List[str]
        :rtype: List[List[str]]
        """
        ma = dict()
        for i in strs:
            k = ''.join(sorted(list(i.lower())))
            if k not in ma:
                ma[k] = []
            ma[k].append(i)
        ans = []
        for i in ma:
            ans.append(ma[i])
        return ans

