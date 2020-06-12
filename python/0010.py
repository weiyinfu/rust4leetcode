import re
class Solution:
    def isMatch(self, s: str, p: str) -> bool:
        p=f"^{p}$"
        x=re.match(p,s)
        return True if x else False