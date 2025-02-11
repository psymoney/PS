from collections import deque

class Solution:
    def removeOccurrences(self, s: str, part: str) -> str:
        n = len(part)
        q = deque([])

        for c in s:
            q.append(c)
            if len(q) >= n:
                comp_str = "".join(list(q)[-n:])
                if comp_str == part:
                    for _ in range(n):
                        q.pop()

        return "".join(list(q))