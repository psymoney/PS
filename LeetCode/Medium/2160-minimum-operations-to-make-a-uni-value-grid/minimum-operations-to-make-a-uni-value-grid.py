class Solution:
    def minOperations(self, grid: List[List[int]], x: int) -> int:
        answer: int = 0
        values: list[int] = []

        for e in grid:
            values += e

        values.sort()
        median: int = values[len(values) // 2]

        for e in values:
            gap: int = abs(e - median)
            q, r = divmod(gap, x)
            if r != 0:
                return -1
            answer += q
        
        return answer
        