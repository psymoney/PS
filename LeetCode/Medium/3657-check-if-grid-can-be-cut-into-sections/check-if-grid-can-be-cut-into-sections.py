class Solution:
    def checkValidCuts(self, n: int, rectangles: List[List[int]]) -> bool:
        self._n = n
        x_coords: list[tuple[int, int]] = []
        y_coords: list[tuple[int, int]] = []

        for sx, sy, ex, ey in rectangles:
            x_coords.append((sx, ex))
            y_coords.append((sy, ey))
        
        x_coords.sort()
        if self.get_number_of_divisions(x_coords) >= 2:
            return True
        y_coords.sort()
        if self.get_number_of_divisions(y_coords) >= 2:
            return True
        return False
    
    def get_number_of_divisions(self, coords: list[tuple[int, int]]) -> int:
        prev_e = coords[0][1]
        count: int = 0

        for s, e in coords[1:]:
            if prev_e <= s:
                count += 1
            prev_e = max(prev_e, e)

        return count