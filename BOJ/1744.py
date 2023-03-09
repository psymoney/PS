import sys
from heapq import heappop, heappush

input = sys.stdin.readline
N = int(input())
PQ = []
MQ = []
for _ in range(N):
    n = int(input())
    if n >= 0:
        heappush(PQ, -n)
    else:
        heappush(MQ, -n)

max_sum = 0

while len(PQ) > 1:
    a, b = heappop(PQ) * (-1), heappop(PQ) * (-1)
    max_sum += max(a * b, a + b)

while len(MQ) > 1:
    a, b = heappop(MQ) * (-1), heappop(MQ) * (-1)
    max_sum += max(a * b, a + b)

if len(PQ) == 1:
    pq = heappop(PQ) * (-1)
    if len(MQ) == 1:
        mq = heappop(MQ) * (-1)
        pq = max(pq * mq, pq + mq)
    max_sum += pq
else:
    if len(MQ) == 1:
        max_sum -= heappop(MQ)
print(max_sum)

