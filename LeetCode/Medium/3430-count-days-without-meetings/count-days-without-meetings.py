class Solution:
    def countDays(self, days: int, meetings: List[List[int]]) -> int:
        if len(meetings) == 1:
            return days - (meetings[0][1] - meetings[0][0] + 1)

        meetings = sorted(meetings, key=lambda x: x[0])
        s, e = None, None

        while len(meetings):
            if not s and not e:
                s, e = meetings.pop(0)
            ns, ne = meetings.pop(0)

            if e < ns - 1:
                days -= (e - s + 1)
                s = ns
                e = ne
            else:
                e = max(e, ne)

        return days - (e - s + 1)