class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        n: int = len(nums)
        count: int = 0
        frequency: dict[int, int] = {}

        for idx, val in enumerate(nums):
            key = idx - val
            count += frequency.get(key, 0)
            frequency[key] = frequency.get(key, 0) + 1

        return n * (n - 1) // 2 - count
        