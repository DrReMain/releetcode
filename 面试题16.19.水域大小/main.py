class Solution:
    def pondSizes(self, land: List[List[int]]) -> List[int]:
        row, col = len(land), len(land[0])

        def dfs(x: int, y: int) -> int:
            if x < 0 or x >= row or y < 0 or y >= col or land[x][y] != 0:
                return 0
            
            land[x][y] = -1
            result = 1
            for dx in [-1, 0, 1]:
                for dy in [-1, 0, 1]:
                    if dx == dy == 0:
                        continue
                    result += dfs(x + dx, y + dy)
            return result
        
        def bfs(x: int, y: int) -> int:
            result = 0
            q = deque([(x, y)])
            land[x][y] = -1

            while q:
                x, y = q.popleft()
                result += 1
                for dx in [-1, 0, 1]:
                    for dy in [-1, 0, 1]:
                        if dx == dy == 0:
                            continue
                        if x + dx < 0 or x + dx >= row or y + dy < 0 or y + dy >= col or land[x + dx][y + dy] != 0:
                            continue
                        land[x + dx][y + dy] = -1
                        q.append((x + dx, y + dy))
            return result

        li = list()
        for i in range(row):
            for j in range(col):
                if land[i][j] == 0:
                    # li.append(dfs(i, j))
                    li.append(bfs(i, j))
        li.sort()
        return li