class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        answer: int = len(nums) * (len(nums) - 1) // 2
        frequency: dict[int, int] = {}
        effective_frequency_keys: set[int] = set()

        for idx, val in enumerate(nums):
            key = idx - val
            frequency.setdefault(key, 0)
            frequency[key] += 1
            if frequency[key] > 1:
                effective_frequency_keys.add(key)

        for key in effective_frequency_keys:
            val = frequency[key]
            answer -= val * (val - 1) // 2
        
        return answer
        