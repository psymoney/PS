class Solution:
    def getHappyString(self, n: int, k: int) -> str:
        if k > 3 * (2 ** (n - 1)):
            return ""
        bag: list[str] = []
        self.make_happy("", n, bag)
        return bag[k - 1]
        
    def make_happy(self, s: str, n: int, bag: list[str]):
        if len(s) == n:
            bag.append(s)
            return
        
        for c in ['a', 'b', 'c']:
            if len(s) and c == s[-1]: 
                continue
            s += c
            self.make_happy(s, n, bag)
            s = s[:-1]
        