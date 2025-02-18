class Solution:
    def smallestNumber(self, pattern: str) -> str:
        ans: str = ""
        partial: str = ""
        for i in range(len(pattern) + 1):
            partial += str(i + 1)
            if i == len(pattern) or pattern[i] == 'I':
                ans += partial[::-1]
                partial = ""
        
        return ans
            
        