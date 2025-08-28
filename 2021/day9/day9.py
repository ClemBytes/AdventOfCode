def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day9/{kind}") as f:
        heightmap = [list(map(int, line.strip())) for line in f]
        return heightmap


def is_low_point(l, c, heightmap):
    # for nl, nc in [(l - 1, c), (l + 1, c), (l, c - 1), (l, c + 1)]:
    #     if 0 <= nl < len(heightmap) and 0 <= nc < len(heightmap[0]):
    #         heightmap[nl][nc] heightmap[l][c]ob
    if l == 0:
        # FIRST LINE
        if c == 0:
            # Point (0,0): only look at (1,0) and (0,1)
            if (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
        elif c == len(heightmap[0]) - 1:
            # Point at the end of first line: only look at (1,c) and (0,c-1)
            if (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1]):
                return True
        else:
            # Point on first line but not corner
            if (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
    elif l == len(heightmap) - 1:
        # LAST LINE
        if c == 0:
            # Only look at (l-1,c) and (l,c+1)
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
        elif c == len(heightmap[0]) - 1:
            # Point at the end of last line: only look at (l-1,c) and (l,c-1)
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1]):
                return True
        else:
            # Point on last line but not corner
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
    else:
        if c == 0:
            # First column
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
        elif c == len(heightmap[0]) - 1:
            # Last column
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1]):
                return True
        else:
        # INSIDE GRID
            if (heightmap[l][c] < heightmap[l-1][c])\
                    and (heightmap[l][c] < heightmap[l+1][c])\
                    and (heightmap[l][c] < heightmap[l][c-1])\
                    and (heightmap[l][c] < heightmap[l][c+1]):
                return True
    return False


def puzzle1(heightmap):
    risk = 0
    for l in range(len(heightmap)):
        for c in range(len(heightmap[l])):
            if is_low_point(l, c, heightmap):
                risk += 1 + heightmap[l][c]
    return risk


def size_bassin(start, heightmap):
    q = [start]
    seen = set()
    compteur = 0
    while q:
        state = q.pop()
        if state in seen:
            continue
        seen.add(state)
        compteur += 1
        l, c = state
        for nl, nc in [(l - 1, c), (l + 1, c), (l, c - 1), (l, c + 1)]:
            if 0 <= nl < len(heightmap) and 0 <= nc < len(heightmap[0]) and heightmap[nl][nc] != 9:
                q.append((nl, nc))
    return compteur


def puzzle2(heightmap):
    sizes = []
    for l in range(len(heightmap)):
        for c in range(len(heightmap[l])):
            if is_low_point(l, c, heightmap):
                sizes.append(size_bassin((l, c), heightmap))
    sizes.sort()
    return sizes[-3] * sizes[-2] * sizes[-1]



def main():
    # Read input
    heightmap = read_input("input")
    #print(heightmap)
    #print(len(heightmap))
    #print(len(heightmap[0]))
    print(puzzle1(heightmap))
    print(puzzle2(heightmap))


main()
