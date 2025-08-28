# Read input
f = open("./input")

'''
groups = []
group = set()
for line in f:
    if line == '\n' and group:
        group = group.union(person)
        groups.append(group)
        group = set()
    else:
        person = set(line.strip())
        group = group.union(person)

group = group.union(person)
'''

groups = []
group = []
for line in f:
    if line == '\n' and group:
        g = group[0]
        for j in range(1, len(group)):
            g = g.intersection(group[j])
        groups.append(g)
        group = []
    else:
        group.append(set(line.strip()))

g = group[0]
for j in range(1, len(group)):
    g = g.intersection(group[j])
groups.append(g)

s = 0
for group in groups:
    s += len(group)

print(s)