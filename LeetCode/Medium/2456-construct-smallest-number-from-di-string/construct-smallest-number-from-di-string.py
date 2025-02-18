class Solution:
    def smallestNumber(self, pattern: str) -> str:
        bag: list[int] = []
        used: list[int] = [0] * 10
        for i in range(1, 10):
            used[i] += 1
            self.backtrack(str(i), 0, pattern, bag, used)
            used[i] -= 1
            if len(bag):
                break

        return str(min(bag))
    
    def backtrack(self, ans: str, n: int, pattern:str, bag: list[int], used: dict[int, int]):
        if n == len(pattern):
            bag.append(int(ans))
            return

        for i in range(1, 10):
            if used[i]:
                continue
            if pattern[n] == 'I':
                if len(ans) and int(ans[-1]) == 9: continue
                if len(ans) and i < int(ans[-1]): continue
                used[i] += 1
                ans += str(i)
                self.backtrack(ans, n+1, pattern, bag, used)
                ans = ans[:-1]
                used[i] -= 1
            else:
                if len(ans) and int(ans[-1]) == 1: continue
                if len(ans) and i > int(ans[-1]): continue
                used[i] += 1
                ans += str(i)
                self.backtrack(ans, n+1, pattern, bag, used)
                ans = ans[:-1]
                used[i] -= 1
            
        