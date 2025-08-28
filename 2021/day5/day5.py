def parse_point(coords):
    return [int(x) for x in coords.split(',')]


def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day5/{kind}") as f:
        vents = []
        for line in f:
            coords1, coords2 = line.strip().split(' -> ')
            vents.append([parse_point(coords1), parse_point(coords2)])
    return vents


def size_grid(vents):
    max_x = max(
        x
        for vent in vents
        for x, _ in vent
    )
    max_y = max(
        y
        for vent in vents
        for _, y in vent
    )
    return max_x, max_y


def overlaps(grid):
    return sum(
        1
        for row in grid
        for val in row
        if val > 1
    )


def puzzle1(vents):
    max_x, max_y = size_grid(vents)
    grid = [[0]*(max_y + 1) for _ in range(max_x + 1)]
    for a, b in vents:
        xa, ya = a
        xb, yb = b
        if xa == xb:
            starty = min(ya, yb)
            stopy = max(ya, yb) + 1
            for y in range(starty, stopy):
                grid[xa][y] += 1
        elif ya == yb:
            startx = min(xa, xb)
            stopx = max(xa, xb) + 1
            for x in range(startx, stopx):
                grid[x][ya] += 1
    return(overlaps(grid))


def puzzle2(vents):
    max_x, max_y = size_grid(vents)
    grid = [[0]*(max_y + 1) for _ in range(max_x + 1)]
    for a, b in vents:
        xa, ya = a
        xb, yb = b

        if xa < xb:
            dx = 1
        elif xa > xb:
            dx = -1
        else:
            dx = 0

        if ya < yb:
            dy = 1
        elif ya > yb:
            dy = -1
        else:
            dy = 0

        x, y = a
        while [x, y] != b:
            grid[x][y] += 1
            x += dx
            y += dy
        grid[x][y] += 1

    return(overlaps(grid))


def main():
    # Read input
    vents = read_input("input")
    #print(vents)
    print(puzzle1(vents))
    print(puzzle2(vents))


main()
