class Solution:
    def countBadPairs(self, nums: List[int]) -> int:
        n: int = len(nums)
        answer: int = n * (n - 1) // 2
        frequency: dict[int, int] = {}

        for idx, val in enumerate(nums):
            key = idx - val
            answer -= frequency.get(key, 0)
            frequency[key] = frequency.get(key, 0) + 1

        return answer
        