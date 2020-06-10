class Solution(object):
    def longestCommonPrefix(self, strs):
        """
        :type strs: List[str]
        :rtype: str
        """
        if not strs:
            return ""
        ml=min(map(lambda x:len(x),strs))
        ans=-1
        for i in range(ml):
            s=set()
            for j in strs:
                s.add(j[i])
            if len(s)!=1:
                ans=i
                break
        if ans==-1:ans=ml
        return strs[0][:ans]