import sys
input = sys.stdin.readline
cases = int(input())

for _ in range(cases):
    n = int(input())
    prices = list(map(int, input().split(' ')))
    profits = 0
    max = 0
    for i in range(n-1, -1, -1):
        if prices[i] > max:
            max = prices[i]
        else:
            profits += max - prices[i]

    print(profits)
  
# 반대로 생각하자. 정방향으로 안되는거 계속 붙잡지 말고
