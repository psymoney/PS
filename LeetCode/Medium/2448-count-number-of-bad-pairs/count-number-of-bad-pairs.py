class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        answer: int = len(nums) * (len(nums) - 1) // 2
        frequency: dict[int, int] = {}

        for idx, val in enumerate(nums):
            key = idx - val
            frequency.setdefault(key, 0)
            frequency[key] += 1

        for val in frequency.values():
            if val > 1:
                answer -= val * (val - 1) // 2
        
        return answer
        