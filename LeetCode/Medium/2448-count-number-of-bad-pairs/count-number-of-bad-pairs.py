class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        answer: int = len(nums) * (len(nums) - 1) // 2
        frequency: dict[int, int] = defaultdict(int)

        for idx, val in enumerate(nums):
            key = idx - val
            frequency[key] += 1

        for val in frequency.values():
            if val > 1:
                answer -= val * (val - 1) // 2
        
        return answer
        