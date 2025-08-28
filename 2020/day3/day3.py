# Read input
f = open("./input")

lines = f.readlines()
l = len(lines)
c = len(lines[0]) - 1
grid = []
for i in range(l):
    line = list(lines[i])
    grid.append(line[:-1])

#grid = grid[:3]
print(grid)
print(l)
print(c)

def aaa(right, down):
    col = 0
    count = 0
    for i in range(down,l,down):
        col = (col+right)%c
        if grid[i][col] == '#':
            count += 1
    return count

print(aaa(1,1)*aaa(3,1)*aaa(5,1)*aaa(7,1)*aaa(1,2))