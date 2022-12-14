from collections import deque

def bfs(map, pos):
    w, h = len(map[0]), len(map)
    q = deque([[pos]])
    seen = set([pos])
    while q:
        path = q.popleft()
        x, y = path[-1]
        if map[y][x] == "E":
            return path
        e = AB.index(map[y][x])
        for x2, y2 in ((x+1,y), (x-1,y), (x,y+1), (x,y-1)):
            if 0 <= x2 < w and 0 <= y2 < h and (x2, y2) not in seen:
                e2 = AB.index(map[y2][x2]) if map[y2][x2] != "E" else 26
                if e2 <= e + 1:
                    q.append(path + [(x2, y2)])
                    seen.add((x2, y2))

data = open("input").read().strip()
AB = "abcdefghijklmnopqrstuvwxyz"
map = [[c for c in line] for line in data.split("\n")]
y, x = [(n, r.index("S")) for n, r in enumerate(map) if "S" in r][0]
y2, x2 = [(n, r.index("E")) for n, r in enumerate(map) if "E" in r][0]
map[y][x] = "a"
print(f"Part 1: {len(bfs(map, (x, y))) - 1}")
starts = [(c, r) for r in range(len(map)) for c in range(len(map[0])) if map[r][c] == "a"]
p2 = [len(bfs(map, pos)) - 1 for pos in starts if bfs(map, pos)]
print(f"Part 2: {min(p2)}")